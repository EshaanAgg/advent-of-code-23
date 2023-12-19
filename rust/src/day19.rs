use std::{collections::HashMap, fs, str::FromStr, vec};

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl PartRange {
    fn get_count(&self) -> usize {
        ((self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)) as usize
    }

    fn get_range(&self, identifier: char) -> (i64, i64) {
        match identifier {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Invalid identifier passed"),
        }
    }

    fn clone_with_change(&self, property: char, bound: char, value: i64) -> PartRange {
        let mut p = self.clone();

        match property {
            'x' => match bound {
                'l' => p.x.0 = value,
                'u' => p.x.1 = value,
                _ => panic!("Invalid bound passed"),
            },
            'm' => match bound {
                'l' => p.m.0 = value,
                'u' => p.m.1 = value,
                _ => panic!("Invalid bound passed"),
            },
            'a' => match bound {
                'l' => p.a.0 = value,
                'u' => p.a.1 = value,
                _ => panic!("Invalid bound passed"),
            },
            's' => match bound {
                'l' => p.s.0 = value,
                'u' => p.s.1 = value,
                _ => panic!("Invalid bound passed"),
            },
            _ => panic!("Invalid property passed"),
        };

        p
    }

    fn apply_rule(&self, rule: &Rule) -> Vec<(PartRange, Option<RuleApplyResult>)> {
        let (low, high) = self.get_range(rule.identifier);

        match rule.operator {
            '>' => {
                if low > rule.value {
                    vec![(
                        self.clone(),
                        Some(RuleApplyResult::get_result(rule.workflow.as_str())),
                    )]
                } else {
                    vec![
                        (
                            self.clone_with_change(rule.identifier, 'l', rule.value + 1),
                            Some(RuleApplyResult::get_result(rule.workflow.as_str())),
                        ),
                        (
                            self.clone_with_change(rule.identifier, 'u', rule.value),
                            None,
                        ),
                    ]
                }
            }
            '<' => {
                if high < rule.value {
                    vec![(
                        self.clone(),
                        Some(RuleApplyResult::get_result(rule.workflow.as_str())),
                    )]
                } else {
                    vec![
                        (
                            self.clone_with_change(rule.identifier, 'l', rule.value),
                            None,
                        ),
                        (
                            self.clone_with_change(rule.identifier, 'u', rule.value - 1),
                            Some(RuleApplyResult::get_result(rule.workflow.as_str())),
                        ),
                    ]
                }
            }
            _ => panic!("Operator not implemented in the apply_rule function"),
        }
    }

    fn apply_workflow(&self, workflow: &Workflow) -> Vec<(PartRange, RuleApplyResult)> {
        let mut results: Vec<(PartRange, RuleApplyResult)> = Vec::new();
        let mut to_process: Vec<PartRange> = vec![self.clone()];

        for rule in workflow.rules.iter() {
            let mut next_to_process: Vec<PartRange> = Vec::new();

            to_process.iter().for_each(|p| {
                p.apply_rule(rule).iter().for_each(|(part_range, next)| {
                    if let Some(next) = next {
                        results.push((*part_range, next.clone()));
                    } else {
                        next_to_process.push(*part_range);
                    }
                })
            });

            to_process = next_to_process;
        }

        for part_range in to_process.iter() {
            results.push((
                *part_range,
                RuleApplyResult::get_result(workflow.next_workflow.as_str()),
            ));
        }

        results
    }

    fn accepted_count(&self, workflows: &HashMap<String, Workflow>) -> usize {
        let mut to_process: Vec<(PartRange, String)> = vec![(self.clone(), "in".to_string())];
        let mut total = 0;

        while to_process.len() != 0 {
            let mut next_to_process: Vec<(PartRange, String)> = Vec::new();

            to_process.iter().for_each(|(p, workflow_name)| {
                p.apply_workflow(
                    workflows
                        .get(workflow_name.as_str())
                        .expect("The workflow with the provided does not exist"),
                )
                .iter()
                .for_each(|(part_range, next)| match next {
                    RuleApplyResult::Accepted => total += part_range.get_count(),
                    RuleApplyResult::Rejected => (),
                    RuleApplyResult::Workflow(next_workflow) => {
                        next_to_process.push((part_range.clone(), next_workflow.clone()))
                    }
                });
            });

            to_process = next_to_process;
        }

        total
    }
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

#[derive(Debug, Clone)]
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

pub fn part2(file_path: &str) -> usize {
    let mut workflows = HashMap::new();

    fs::read_to_string(file_path)
        .expect("Failed to read the file")
        .split("\n\n")
        .next()
        .expect("The input must have workflows")
        .lines()
        .for_each(|line| {
            let workflow = line
                .parse::<Workflow>()
                .expect("Failed to parse a workflow");
            workflows.insert(workflow.name.clone(), workflow);
        });

    PartRange {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    }
    .accepted_count(&workflows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("../data/19/test.txt"), 19114);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2("../data/19/test.txt"), 167409079868000);
    }
}
