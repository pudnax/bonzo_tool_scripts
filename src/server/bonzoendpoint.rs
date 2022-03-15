#[derive(Debug, Clone)]
pub struct BonzoEndpoint {
    room: String,
    user: Option<String>,
}

impl BonzoEndpoint {
    pub fn empty() -> Self {
        BonzoEndpoint {
            room: String::default(),
            user: None,
        }
    }
    pub fn can_send_to(&self, other: &BonzoEndpoint) -> bool {
        other.room == self.room && (other.user == None || other.user == self.user)
    }

    pub fn json_filename(&self) -> Result<String, ()> {
        match &self.user {
            Some(user) => Ok(self.room.to_owned() + "_" + user + ".json"),
            _ => Err(()),
        }
    }
    pub fn parse_resource(query: &str) -> Result<BonzoEndpoint, String> {
        if query.chars().nth(0).unwrap() != '/' {
            Err(String::from("Not starting with /"))
        } else {
            let splited_query = query.split("/").filter(|x| *x != "").collect::<Vec<&str>>();
            match splited_query.len() {
                1 => Ok(BonzoEndpoint {
                    room: String::from(splited_query[0]),
                    user: None,
                }),
                2 => Ok(BonzoEndpoint {
                    room: String::from(splited_query[0]),
                    user: Some(String::from(splited_query[1])),
                }),
                _ => Err(String::from("Path not correct")),
            }
        }
    }
}
#[test]
fn test_parse_bonzoendpoint_struct() {
    let faulty = "toto/toto";
    let faulty = BonzoEndpoint::parse_resource(faulty);
    assert!(faulty.is_err());

    let ok = "/abcd/toto";
    let ok = BonzoEndpoint::parse_resource(ok);
    assert!(ok.is_ok());
    let ok = ok.unwrap();
    assert!(ok.user == Some(String::from("toto")));
    assert!(ok.room == String::from("abcd"));

    let ok = "/abcd/toto/";
    let ok = BonzoEndpoint::parse_resource(ok);
    assert!(ok.is_ok());
    let ok = ok.unwrap();
    assert!(ok.user == Some(String::from("toto")));
    assert!(ok.room == String::from("abcd"));

    let ok = "/abcd/";
    let ok = BonzoEndpoint::parse_resource(ok);
    assert!(ok.is_ok());
    let ok = ok.unwrap();
    assert!(ok.user == None);
    assert!(ok.room == String::from("abcd"));

    let ok = "/abcd";
    let ok = BonzoEndpoint::parse_resource(ok);
    assert!(ok.is_ok());
    let ok = ok.unwrap();
    assert!(ok.user == None);
    assert!(ok.room == String::from("abcd"));
}
#[test]
fn test_can_send_to() {
    let user_ep = "/lol/pouet";
    let room_ep = "/lol/";
    let user2_ep = "/lol/toto";
    let user3_ep = "/ttt/toto";

    let user_ep = BonzoEndpoint::parse_resource(user_ep).unwrap();
    let room_ep = BonzoEndpoint::parse_resource(room_ep).unwrap();
    let user2_ep = BonzoEndpoint::parse_resource(user2_ep).unwrap();
    let user3_ep = BonzoEndpoint::parse_resource(user3_ep).unwrap();

    assert!(user_ep.can_send_to(&room_ep));
    assert!(
        !room_ep.can_send_to(&user_ep),
        "Should not be able to send from room to endpoint"
    );

    assert!(user2_ep.can_send_to(&room_ep));
    assert!(!user3_ep.can_send_to(&room_ep));
}
