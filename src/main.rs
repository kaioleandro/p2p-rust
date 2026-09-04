use std::{error::Error, time::Duration};

use chrono::Local;
use futures::StreamExt;
use libp2p::{
    gossipsub,
    mdns,
    noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp,
    yamux,
};
use tokio::{
    io::{self, AsyncBufReadExt},
    select,
};

#[derive(NetworkBehaviour)]
struct Behaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            let message_authenticity = gossipsub::MessageAuthenticity::Signed(key.clone());

            let config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10))
                .build()
                .expect("Configuração válida");

            let mut gossipsub = gossipsub::Behaviour::new(message_authenticity, config)
                .expect("Gossipsub criado");

            let topic = gossipsub::IdentTopic::new("chat");

            gossipsub.subscribe(&topic).expect("Inscrição no tópico");

            Ok(Behaviour {
                gossipsub,
                mdns: mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?,
            })
        })?
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let topic = gossipsub::IdentTopic::new("chat");

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    println!("Digite seu nome para entrar no chat:");
    let nome = loop {
        if let Some(linha) = stdin.next_line().await? {
            let linha = linha.trim().to_string();
            if !linha.is_empty() {
                break linha;
            }
        }
        println!("Nome não pode ser vazio, tente novamente:");
    };

    println!("\nBem-vindo, {nome}! Aguardando conexões e mensagens...\n");

    loop {
        select! {
            Ok(Some(line)) = stdin.next_line() => {
                let agora = Local::now().format("%d/%m/%Y %H:%M:%S");
                let mensagem = format!("[{agora}] {nome}: {line}");

                if let Err(error) = swarm
                    .behaviour_mut()
                    .gossipsub
                    .publish(topic.clone(), mensagem.as_bytes())
                {
                    println!("Erro ao enviar: {error:?}");
                }
            }

            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                    for (peer_id, _) in peers {
                        println!("Peer encontrado: {peer_id}");
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                }

                SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Expired(peers))) => {
                    for (peer_id, _) in peers {
                        println!("Peer desconectado: {peer_id}");
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                }

                SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    message,
                    ..
                })) => {
                    println!("\n{}", String::from_utf8_lossy(&message.data));
                }

                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Escutando em: {address}");
                }

                _ => {}
            }
        }
    }
}