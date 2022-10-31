//! #DRC_lib
//!
//! Library for designing decide-rs clients
pub mod cli_param {

    use decide_protocol::{
        ComponentName,
        ComponentRequest,
        proto::{ComponentParams, reply, Reply},
        REQ_ENDPOINT,
        Request,
        RequestType,
    };
    use house_light::proto as hl_proto;
    use peckboard::proto as pb_proto;
    use stepper_motor::proto as sm_proto;
    use sound_alsa::proto as playback_proto;
    use prost;
    use prost::Message;
    use prost_types::Any;
    use tmq::{Context, Multipart, Result};

    const HS_PARAMS_TYPE_URL: &'static str = "melizalab.org/proto/house_light_params";
    const PL_PARAMS_TYPE_URL: &'static str = "melizalab.org/proto/led_params";
    const PK_PARAMS_TYPE_URL: &'static str = "melizalab.org/proto/key_params";
    const SM_PARAMS_TYPE_URL: &'static str = "melizalab.org/proto/stepper_params";
    const PLB_PARAMS_TYPE_URL: &'static str = "melizalab.org/proto/sound_alsa_params";

    async fn send_request(message: Request) -> Result<reply::Result> {
        let ctx = Context::new();
        //tracing::trace!("trying to connect");
        let req_sock = tmq::request(&ctx).connect(REQ_ENDPOINT)?;
        tracing::trace!("connected");

        let message = Multipart::from(message);
        //tracing::trace!("trying to send message");
        let reply_sock = req_sock.send(message).await?;
        tracing::trace!("sent message");
        let (multipart, _req) = reply_sock.recv().await?;
        tracing::trace!("received reply");
        let reply = Reply::from(multipart);
        tracing::trace!("reply");
        Ok(reply.result.unwrap())
    }

    /// Init House Light Params
    /// interval = wait duration in secs
    /// Recommended 300
    pub async fn set_hl(interval:i64) -> Result<()>{
        let params = prost_types::Any {
            type_url: String::from(HS_PARAMS_TYPE_URL),
            value: hl_proto::Params {
                clock_interval:interval
            }.encode_to_vec(),
        };
        let params_message = ComponentParams {
            parameters: Some(params.clone().into()),
        };
        let request = Request {
            request_type: RequestType::Component(ComponentRequest::SetParameters),
            component: Some(ComponentName(String::from("house-light"))),
            body: params_message.encode_to_vec(),
        };
        let result = send_request(request).await.unwrap();
        assert_eq!(result, reply::Result::Ok(()));
        let request = Request {
            request_type: RequestType::Component(ComponentRequest::GetParameters),
            component: Some(ComponentName::from("house-light")),
            body: vec![],
        };
        let result = send_request(request).await.unwrap();
        assert_eq!(result, reply::Result::Params(params.into()));
        Ok(())
    }
    /// Init PeckBoard LEDs Params
    /// No Params specified
    pub async fn set_pbled() -> Result<()>{
        let params = Any {
            type_url: String::from(PL_PARAMS_TYPE_URL),
            value: pb_proto::LedParams {
            }.encode_to_vec(),
        };
        let params_message = ComponentParams {
            parameters: Some(params.clone().into()),
        };

        components = ["peck-leds-left","peck-leds-right","peck-leds-center"];

        // Peckboard Components have empty Params
        for c in components {
            let request = Request {
                request_type: RequestType::Component(ComponentRequest::SetParameters),
                component: Some(ComponentName(String::from(c))),
                body: params_message.encode_to_vec(),
            };
            let result = send_request(request).await.unwrap();
            assert_eq!(result, reply::Result::Ok(()));
            let request = Request {
                request_type: RequestType::Component(ComponentRequest::GetParameters),
                component: Some(ComponentName::from(c)),
                body: vec![],
            };
            let result = send_request(request).await.unwrap();
            assert_eq!(result, reply::Result::Params(params.into()));
        };
        Ok(())
    }

    /// Init PeckBoard Key Params
    /// No Params specified
    pub async fn set_pbkey() -> Result<()>{
        let led_params = Any {
            type_url: String::from(PK_PARAMS_TYPE_URL),
            value: pb_proto::KeyParams {
            }.encode_to_vec(),
        };
        let params_message = ComponentParams {
            parameters: Some(params.clone().into()),
        };
        // Peckboard Components have empty Params
        let request = Request {
            request_type: RequestType::Component(ComponentRequest::SetParameters),
            component: Some(ComponentName(String::from("peck-keys"))),
            body: params_message.encode_to_vec(),
        };
        let result = send_request(request).await.unwrap();
        assert_eq!(result, reply::Result::Ok(()));
        let request = Request {
            request_type: RequestType::Component(ComponentRequest::GetParameters),
            component: Some(ComponentName::from("peck-keys")),
            body: vec![],
        };
        let result = send_request(request).await.unwrap();
        assert_eq!(result, reply::Result::Params(params));
        Ok(())
    }

    /// Init Stepper Motor Params
    /// timeout = duration to run motor after 1 signal in ms
    /// Recommended 1000
    pub async fn set_sm(timeout:u64) -> Result<()>{
        let params = Any {
            type_url: String::from(SM_PARAMS_TYPE_URL),
            value: sm_proto::Params {
                timeout
            }.encode_to_vec(),
        };
        let params_message = ComponentParams {
            parameters: Some(params.clone().into()),
        };
        let request = Request {
            request_type: RequestType::Component(ComponentRequest::SetParameters),
            component: Some(ComponentName(String::from("stepper-motor"))),
            body: params_message.encode_to_vec(),
        };
        let result = send_request(request).await.unwrap();
        assert_eq!(result, reply::Result::Ok(()));
        let request = Request {
            request_type: RequestType::Component(ComponentRequest::GetParameters),
            component: Some(ComponentName::from("stepper-motor")),
            body: vec![],
        };
        let result = send_request(request).await.unwrap();
        assert_eq!(result, reply::Result::Params(params.into()));
        Ok(())
    }

    /// Init Audio Playback Params
    /// No Params specified
    pub async fn set_playback() -> Result<()>{
        let params = Any {
            type_url: String::from(PLB_PARAMS_TYPE_URL),
            value: pb_proto::Params {
            }.encode_to_vec(),
        };
        let params_message = ComponentParams {
            parameters: Some(params.clone().into()),
        };
        let request = Request {
            request_type: RequestType::Component(ComponentRequest::SetParameters),
            component: Some(ComponentName(String::from("sound-alsa"))),
            body: params_message.encode_to_vec(),
        };
        let result = send_request(request).await.unwrap();
        assert_eq!(result, reply::Result::Ok(()));
        let request = Request {
            request_type: RequestType::Component(ComponentRequest::GetParameters),
            component: Some(ComponentName::from("sound-alsa")),
            body: vec![],
        };
        let result = send_request(request).await.unwrap();
        assert_eq!(result, reply::Result::Params(params.into()));
        Ok(())
    }

}

pub mod cli_state {
    use decide_protocol::{
        ComponentName,
        ComponentRequest,
        proto::{reply, Reply, StateChange},
        REQ_ENDPOINT,
        Request,
        RequestType,
    };
    use house_light::proto as hl_proto;
    use peckboard::proto as pb_proto;
    use stepper_motor::proto as sm_proto;
    use sound_alsa::proto as playback_proto;
    use prost;
    use prost_types::Any;
    use prost::Message;
    use tmq::{Context, Multipart, Result};

    const HS_STATE_TYPE_URL: &'static str = "melizalab.org/proto/house_light_state";
    const PL_STATE_TYPE_URL: &'static str = "melizalab.org/proto/led_state";
    const PK_STATE_TYPE_URL: &'static str = "melizalab.org/proto/key_state";
    const SM_STATE_TYPE_URL: &'static str = "melizalab.org/proto/stepper_state";
    const PLB_STATE_TYPE_URL: &'static str = "melizalab.org/proto/sound_alsa_state";

    pub async fn set_hs(switch:bool, light_override:bool, fake_clock:bool, brightness:i32) {
        let state = Any {
            type_url: String::from(HS_STATE_TYPE_URL),
            value: hl_proto::State {
                switch,
                light_override,
                fake_clock,
                brightness,
            }.encode_to_vec(),
        };
        let state_message = StateChange {
            state: Some(state.clone().into()),
        };
        let request = Request {
            request_type: RequestType::Component(ComponentRequest::ChangeState),
            component: Some(ComponentName::from("house-light")),
            body: state_message.encode_to_vec(),
        };
        // the subscriber must be initialized before the state change is
        // sent because the publish socket doesn't buffer messages
        let result = send_request(request).await.unwrap();
        assert_eq!(result, reply::Result::Ok(()));
    }
    pub async fn set_pl(loc:&str, led_state:&str) {
        let state = Any {
            type_url: String::from(PL_STATE_TYPE_URL),
            value: pb_proto::LedState {
                led_state: String::from(led_state)
            }.encode_to_vec(),
        };
        let state_message = StateChange {
            state: Some(state.into()),
        };
        let request = Request {
            request_type: RequestType::Component(ComponentRequest::ChangeState),
            component: Some(ComponentName::from(loc)),
            body: state_message.encode_to_vec(),
        };
        // the subscriber must be initialized before the state change is
        // sent because the publish socket doesn't buffer messages
        let result = send_request(request).await.unwrap();
        assert_eq!(result, reply::Result::Ok(()));
    }
    pub async fn set_sm(switch:bool, on: bool, direction:bool) {
        let state = Any {
            type_url: String::from(SM_STATE_TYPE_URL),
            value: sm_proto::State {
                switch,
                on,
                direction,
            }.encode_to_vec(),
        };
        let state_message = StateChange {
            state: Some(state.into()),
        };
        let request = Request {
            request_type: RequestType::Component(ComponentRequest::ChangeState),
            component: Some(ComponentName::from("stepper-motor")),
            body: state_message.encode_to_vec(),
        };
        // the subscriber must be initialized before the state change is
        // sent because the publish socket doesn't buffer messages
        let result = send_request(request).await.unwrap();
        assert_eq!(result, reply::Result::Ok(()));
    }
    pub async fn set_playback(id:&str, playback:i32) {
        let state = Any {
            type_url: String::from(PLB_STATE_TYPE_URL),
            value: playback_proto::State {
                audio_id: id.to_string(),
                playback,
                elapsed: None
            }.encode_to_vec(),
        };
        let state_message = StateChange {
            state: Some(state.into()),
        };
        let request = Request {
            request_type: RequestType::Component(ComponentRequest::ChangeState),
            component: Some(ComponentName::from("sound-alsa")),
            body: state_message.encode_to_vec(),
        };
        // the subscriber must be initialized before the state change is
        // sent because the publish socket doesn't buffer messages
        let result = send_request(request).await.unwrap();
        assert_eq!(result, reply::Result::Ok(()));
    }

}

