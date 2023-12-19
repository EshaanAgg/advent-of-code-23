use std::{collections::HashMap, fs, str::FromStr};

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, m, a, s) = scan_fmt!(s, "{{x={},m={},a={},s={}}}", i64, i64, i64, i64)
            .expect("Failed to parse a part from the input line");
        Ok(Part { x, m, a, s })
    }
}

impl Part {
    fn get_value(&self, identifier: char) -> i64 {
        match identifier {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Invalid identifier passed"),
        }
    }

    fn get_score(&self) -> usize {
        (self.x + self.m + self.a + self.s) as usize
    }

    fn apply_rule(&self, rule: &Rule) -> Option<RuleApplyResult> {
        let value = self.get_value(rule.identifier);
        let result = match rule.operator {
            '>' => value > rule.value,
            '<' => value < rule.value,
            _ => panic!("Operator not implemented in the apply_rule function"),
        };

        if result {
            Some(RuleApplyResult::get_result(rule.workflow.as_str()))
        } else {
            None
        }
    }

    fn apply_workflow(&self, workflow: &Workflow) -> RuleApplyResult {
        for rule in workflow.rules.iter() {
            if let Some(result) = self.apply_rule(rule) {
                return result;
            }
        }
        RuleApplyResult::get_result(workflow.next_workflow.as_str())
    }

    fn accepted(&self, workflows: &HashMap<String, Workflow>) -> bool {
        let mut workflow = workflows.get("in").expect("Workflow in must exist");

        loop {
            match self.apply_workflow(workflow) {
                RuleApplyResult::Accepted => return true,
                RuleApplyResult::Rejected => return false,
                RuleApplyResult::Workflow(next_workflow) => {
                    workflow = workflows.get(next_workflow.as_str()).expect(
                        "Workflow provided in the next parameter of another workflow must exist",
                    );
                }
            }
        }
    }
}

#[derive(Debug)]
enum RuleApplyResult {
    Workflow(String),
    Accepted,
    Rejected,
}

impl RuleApplyResult {
    fn get_result(str: &str) -> RuleApplyResult {
        match str {
            "A" => RuleApplyResult::Accepted,
            "R" => RuleApplyResult::Rejected,
            _ => RuleApplyResult::Workflow(str.to_string()),
        }
    }
}

#[derive(Debug)]
struct Rule {
    identifier: char,
    operator: char,
    value: i64,
    workflow: String,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let delimiter = s.find(":").expect("Every rule must have a delimiter");

        Ok(Rule {
            identifier: s
                .chars()
                .next()
                .expect("Every rule must have an identifier"),
            operator: s.chars().nth(1).expect("Every rule must have an operator"),
            value: s[2..delimiter]
                .parse::<i64>()
                .expect("Failed to parse a rule value"),
            workflow: s[delimiter + 1..].to_string(),
        })
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    next_workflow: String,
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opening_brace = s
            .find('{')
            .expect("Every workflow must have an opening brace");
        let closing_brace = s
            .find('}')
            .expect("Every workflow must have a closing brace");

        let mut rule_split = s[opening_brace + 1..closing_brace]
            .split(",")
            .collect::<Vec<&str>>();

        Ok(Workflow {
            name: s[..opening_brace].to_string(),
            next_workflow: rule_split
                .pop()
                .expect("Every workflow must have a next workflow")
                .to_string(),
            rules: rule_split
                .iter()
                .map(|rule| rule.parse::<Rule>().expect("Failed to parse a rule"))
                .collect(),
        })
    }
}

pub fn part1(file_path: &str) -> usize {
    let content = fs::read_to_string(file_path).expect("Failed to read the file");
    let mut parts = content.split("\n\n");

    let mut workflows = HashMap::new();
    parts
        .next()
        .expect("The input must have workflows")
        .lines()
        .for_each(|line| {
            let workflow = line
                .parse::<Workflow>()
                .expect("Failed to parse a workflow");
            workflows.insert(workflow.name.clone(), workflow);
        });

    parts
        .next()
        .expect("The input must have associated parts")
        .lines()
        .map(|line| line.parse::<Part>().expect("Failed to parse a part"))
        .filter(|part| part.accepted(&workflows))
        .map(|p| p.get_score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("../data/19/test.txt"), 19114);
    }

    // #[test]
    // fn test_2() {
    //     assert_eq!(part2("../data/17/test1.txt"), 94);
    //     assert_eq!(part2("../data/17/test2.txt"), 71);
    // }
}
