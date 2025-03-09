use std::fs;


pub fn length_limit() {
    let file_path = "input1.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let total_charge = calculate_total_charge(&contents);
    println!("Result puzzle 1: {}", total_charge);
}

fn is_valid_sms(msg: &str) -> bool {
    msg.len() <= 160 // bytes
}

fn is_valid_tweet(msg: &str) -> bool {
    msg.chars().count() <= 140 // characters
}

fn calculate_charge(msg: &str) -> u32 {
    let valid_sms = is_valid_sms(msg);
    let valid_tweet = is_valid_tweet(msg);
    if valid_sms && valid_tweet {
        13
    } else if valid_sms {
        11
    } else if valid_tweet {
        7
    } else {
        0
    }
}

fn calculate_total_charge(msg: &str) -> u32 {
    let mut total_charge: u32 = 0;
    msg.split("\n").for_each(|one_msg| {
        total_charge += calculate_charge(one_msg);
    });
    total_charge
}



#[cfg(test)]
mod tests {
    use super::*;

    const MSG_0: &str = "néztek bele az „ártatlan lapocskába“, mint ahogy belenézetlen mondták ki rá a halálos itéletet a sajtó csupa 20–30 éves birái s egyben hóhérai.";
    const MSG_1: &str = "livres, et la Columbiad Rodman ne dépense que cent soixante livres de poudre pour envoyer à six milles son boulet d'une demi-tonne.  Ces";
    const MSG_2: &str = "Люди должны были тамъ и сямъ жить въ палаткахъ, да и мы не были помѣщены въ посольскомъ дворѣ, который также сгорѣлъ, а въ двухъ деревянныхъ";
    const MSG_3: &str = "Han hade icke träffat Märta sedan Arvidsons middag, och det hade gått nära en vecka sedan dess. Han hade dagligen promenerat på de gator, där";
    const MSG_COMBINED: &str = "néztek bele az „ártatlan lapocskába“, mint ahogy belenézetlen mondták ki rá a halálos itéletet a sajtó csupa 20–30 éves birái s egyben hóhérai.
livres, et la Columbiad Rodman ne dépense que cent soixante livres de poudre pour envoyer à six milles son boulet d'une demi-tonne.  Ces
Люди должны были тамъ и сямъ жить въ палаткахъ, да и мы не были помѣщены въ посольскомъ дворѣ, который также сгорѣлъ, а въ двухъ деревянныхъ
Han hade icke träffat Märta sedan Arvidsons middag, och det hade gått nära en vecka sedan dess. Han hade dagligen promenerat på de gator, där";

    #[test]
    fn calculate_charge_correct() {
        assert_eq!(calculate_charge(MSG_0), 0);
        assert_eq!(calculate_charge(MSG_1), 13);
        assert_eq!(calculate_charge(MSG_2), 7);
        assert_eq!(calculate_charge(MSG_3), 11);
    }

    #[test]
    fn calculate_total_charge_correct() {
        assert_eq!(calculate_total_charge(MSG_COMBINED), 31);
    }
}