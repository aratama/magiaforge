use bevy::prelude::*;
use ewebsock::{WsReceiver, WsSender};

pub struct GameClientResource {
    sender: WsSender,
    receiver: WsReceiver,
}

pub struct GameClientPlugin;

fn update(mut res: NonSendMut<GameClientResource>, keys: Res<ButtonInput<KeyCode>>) {
    let receiver = &res.receiver;

    while let Some(event) = receiver.try_recv() {
        println!("Received {:?}", event);
    }

    if keys.just_pressed(KeyCode::Space) {
        res.sender.send(ewebsock::WsMessage::Text("Hello!".into()));
    }
}

impl Plugin for GameClientPlugin {
    fn build(&self, app: &mut App) {
        let options = ewebsock::Options::default();
        // see documentation for more options
        let (sender, receiver) = ewebsock::connect(
            "https://magia-server-38847751193.asia-northeast1.run.app",
            options,
        )
        .unwrap();

        app.add_systems(FixedUpdate, update);
        app.insert_non_send_resource(GameClientResource { sender, receiver });
    }
}
