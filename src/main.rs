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
    min: u16,
    max: u16,
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

enum Goal {
    WeightLoss,
    Maintenance,
    GainWeight,
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

impl fmt::Display for Goal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let goal = match *self {
            Goal::WeightLoss => "losing weight",
            Goal::Maintenance => "maintaining weight",
            Goal::GainWeight => "gaining weight (lean mass)",
        };

        write!(f, "{}", goal)
    }
}

impl fmt::Display for CaloricIntake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{} kcal", self.min, self.max)
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

    let caloric_treshold = caloric_treshold(luca);

    println!("Luca's caloric treshold is {}", caloric_treshold);

    let goal = Goal::WeightLoss;

    println!(
        "Luca's caloric intake should be {} for {}",
        caloric_intake(caloric_treshold, &goal),
        goal
    );
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

fn caloric_intake(caloric_treshold: f32, goal: &Goal) -> CaloricIntake {
    let caloric_treshold = caloric_treshold as u16;
    let mut caloric_intake = CaloricIntake { min: 0, max: 0 };

    match *goal {
        Goal::WeightLoss => {
            caloric_intake.min = caloric_treshold - 1000;
            caloric_intake.max = caloric_treshold - 500;
        }
        Goal::Maintenance => {
            caloric_intake.min = caloric_treshold;
            caloric_intake.max = caloric_treshold;
        }
        Goal::GainWeight => {
            caloric_intake.min = caloric_treshold + 250;
            caloric_intake.max = caloric_treshold + 500;
        }
    }

    caloric_intake
}
