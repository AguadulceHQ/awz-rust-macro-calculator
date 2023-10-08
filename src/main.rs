use std::fmt;

struct Person {
    name: String,
    age: u8,
    gender: Gender,
    height: u8,
    weight: u8,
    activty: Activity,
}

struct CaloricIntake {
    min: u8,
    max: u8,
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

    println!("Luca's caloric treshold is {}", caloric_treshold(luca));
}

fn caloric_treshold(person: Person) -> f32 {
    const MEN_MULTIPLIERS: (f32, f32, f32, f32) = (88.362, 13.397, 4.799, 5.677);
    const FEMALE_MULTIPLIERS: (f32, f32, f32, f32) = (447.593, 9.247, 3.098, 4.330);
    const ACTIVITY_MULTIPLIERS: (f32, f32, f32, f32, f32) = (1.2, 1.375, 1.55, 1.725, 1.9);

    let basal_metabolic_rate;

    match person.gender {
        Gender::Male => {
            basal_metabolic_rate = MEN_MULTIPLIERS.0
                + MEN_MULTIPLIERS.1 * person.weight as f32
                + MEN_MULTIPLIERS.2 * person.height as f32
                - MEN_MULTIPLIERS.3 * person.age as f32;
        }
        Gender::Female => {
            basal_metabolic_rate = FEMALE_MULTIPLIERS.0
                + FEMALE_MULTIPLIERS.1 * person.weight as f32
                + FEMALE_MULTIPLIERS.2 * person.height as f32
                - FEMALE_MULTIPLIERS.3 * person.age as f32;
        }
    }

    let activity_multiplier;

    match person.activty {
        Activity::LightlyActive => {
            activity_multiplier = ACTIVITY_MULTIPLIERS.0;
        }
        Activity::ModeratelyActive => {
            activity_multiplier = ACTIVITY_MULTIPLIERS.1;
        }
        Activity::Sedentary => {
            activity_multiplier = ACTIVITY_MULTIPLIERS.2;
        }
        Activity::SuperActive => {
            activity_multiplier = ACTIVITY_MULTIPLIERS.3;
        }
        Activity::VeryActive => {
            activity_multiplier = ACTIVITY_MULTIPLIERS.4;
        }
    }

    basal_metabolic_rate * activity_multiplier
}
