struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            name => Some(name.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    let company_vec = vec![
        Company::new("Tesla", "Elon"),
        Company::new("Mazda", "Mazda sang"),
        Company::new("MALTS", ""),
    ];

    let mut results_vec = vec![];

    company_vec.iter().for_each(|company| {
        results_vec.push(company.get_ceo().ok_or_else(|| {
            let err_msg = format!("No CEO found for {}", company.name);
            err_msg
        }))
    });

    for item in results_vec {
        println!("{:?}", item);
    }
}
