use std::io::{BufRead, Write};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use maelstrom::{
    message::{InitializationMessage, InitializationPayload, Message},
    node::Node
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum EchoRequest {
    Echo { echo: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum EchoResponse {
    EchoOk { echo: String }
}

fn initialize_node(line: String, stdout: &mut impl Write) -> anyhow::Result<Node> {
    let init_message: InitializationMessage = line.parse()?;
    let mut node: Node = Node::new(&init_message)?;
    node.respond_to(&init_message)
        .with_payload(InitializationPayload::InitOk)
        .build()
        .write_to(stdout)
        .context("unable to write message to stdout")?;

    Ok(node)
}

pub fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let mut lines = stdin.lines();
    let mut stdout = std::io::stdout().lock();

    let init_line = lines
        .next()
        .expect("an initialization message")
        .context("unable to read line from stdin")?;

    let mut node = initialize_node(init_line, &mut stdout)?;
    for line in lines {
        let line = line.context("unable to read line from stdin")?;
        let message: Message<EchoRequest> = line.parse()?;
        let EchoRequest::Echo { echo } = message.payload();
        node.respond_to(&message)
            .with_payload(EchoResponse::EchoOk {
                echo: echo.to_string(),
            })
            .build()
            .write_to(&mut stdout)
            .context("unable to write message to stdout")?;
    }

    Ok(())
}
