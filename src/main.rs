use std::fmt;

struct Person {
    name: String,
    age: u8,
    gender: Gender,
    height: u8,
    weight: u8,
    activty: Activity,
}

enum Gender {
    Female,
    Male,
}

enum Activity {
    Sedentary,
    LightlyActive,
    ModeratelyActive,
    VeryActive,
    SuperActive,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} is a {} years old {}, {} cms tall that weighs {}kg with an activity level of {} 📝",
            self.name, self.age, self.gender, self.height, self.weight, self.activty
        )
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let gender = match *self {
            Gender::Male => "male",
            Gender::Female => "female",
        };

        write!(f, "{}", gender)
    }
}

impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let activity = match *self {
            Activity::Sedentary => "sedentary",
            Activity::LightlyActive => "lightly active",
            Activity::ModeratelyActive => "moderately active",
            Activity::SuperActive => "super active",
            Activity::VeryActive => "very active",
        };

        write!(f, "{}", activity)
    }
}

fn main() {
    let luca = Person {
        name: String::from("Luca"),
        age: 33,
        gender: Gender::Male,
        height: 173,
        weight: 70,
        activty: Activity::SuperActive,
    };

    println!("{}", luca);
}
