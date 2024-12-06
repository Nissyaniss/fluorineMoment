use dioxus::prelude::*;

#[derive(Clone)]
struct Question {
  context: String,
  question: String,
  answers: Vec<String>,
  correct_answer: usize,
}

#[component]
pub fn Qcm() -> Element {
  let questions = use_signal(|| vec![
    Question {
      context: "L'océan agit comme une mémoire du climat en captant la chaleur.".to_string(),
      question: "Quel organe humain pourrait remplir un rôle similaire en régulant la température du corps ?".to_string(),
      answers: vec![
        "Coeur".to_string(),
        "Peau".to_string(),
        "Poumons".to_string(),
        "Cerveau".to_string(),
      ],
      correct_answer: 1,
    },
    Question {
      context: "Les courants océaniques, comme le Gulf Stream, transportent de la chaleur et des nutriments à travers la planète.".to_string(),
      question: "Quel système du corps humain peut être comparé à cette fonction de transport ?".to_string(),
      answers: vec![
        "Cerveau".to_string(),
        "Le système circulatoire".to_string(),
        "Poumons".to_string(),
        "Coeur".to_string(),
      ],
      correct_answer: 1,
    },
    Question {
      context: "Lorsque la surface de l’océan se réchauffe, les échanges de gaz (comme l’oxygène) entre les profondeurs et la surface diminuent, affectant la faune marine.".to_string(),
      question: "Quel système corporel est responsable d’échanges gazeux similaires dans le corps humain ?".to_string(),
      answers: vec![
        "Coeur".to_string(),
        "Le système circulatoire".to_string(),
        "Poumons".to_string(),
        "Cerveau".to_string(),
      ],
      correct_answer: 2,
    },
    Question {
      context: "Le phytoplancton est la base de la chaîne alimentaire marine, produisant l'énergie pour d'autres organismes.".to_string(),
      question: "À quelle structure ou cellule dans le corps humain pourrait-on comparer le rôle essentiel du phytoplancton ?".to_string(),
      answers: vec![
        "Coeur".to_string(),
        "Le système circulatoire".to_string(),
        "Les cellules".to_string(),
        "Cerveau".to_string(),
      ],
      correct_answer: 2,
    },
    Question {
      context: "Une partie du phytoplancton en décomposition est emprisonnée au fond de l’océan, stockant du CO₂ pendant des milliers d’années.".to_string(),
      question: "Quelle fonction corporelle ou organe humain joue un rôle équivalent en stockant des déchets ou des substances ?".to_string(),
      answers: vec![
        "Coeur".to_string(),
        "Le système circulatoire".to_string(),
        "Les graisses (tissu adipeux)".to_string(),
        "Cerveau".to_string(),
      ],
      correct_answer: 2,
    },
    Question {
      context: "L'acidification des océans modifie leur équilibre chimique et affecte les écosystèmes marins.".to_string(),
      question: "Quel organe humain pourrait être affecté de manière similaire par un déséquilibre chimique dans le corps ?".to_string(),
      answers: vec![
        "Coeur".to_string(),
        "Le système circulatoire".to_string(),
        "Le sang".to_string(),
        "Cerveau".to_string(),
      ],
      correct_answer: 2,
    },
  ]);

  let mut current_question_index = use_signal(|| 0);
  let mut current_question = use_signal(|| questions.read()[0].clone());
  let mut correct_answers = use_signal(|| 0);

  let mut is_current_answer_correct = use_signal(|| false);
  let mut show_modal = use_signal(|| false);

  rsx! {
    if *show_modal.read() {
      div {
        class: "fixed inset-0 bg-black bg-opacity-20 flex items-center justify-center",
        div {
          class: "bg-white p-6",

          p {
            if *is_current_answer_correct.read() {
              "Vrai !"
            } else {
              "Et non..."
            }
          }

          button {
            onclick: move |_| {
              show_modal.set(false);
            },
            "Ok !"
          }
        }
      }
    }

    main {
      class: "flex flex-col gap-2 container mx-auto px-6 py-12",

      p {
        class: "text-4xl mb-6",
        "{current_question_index+1}/6"
      }

      div {
        class: "flex flex-col gap-2 mb-8",

        p {
          class: "font-[Arial]",
          "{current_question.read().context}"
        }
        p {
          class: "font-[Arial] font-bold",
          "{current_question.read().question}"
        }
      }

      div {
        class: "flex flex-col gap-2 items-start",

        for (index, answer) in current_question.read().answers.iter().enumerate() {
          button {
            class: "px-3 py-1.5 opacity-80 hover:opacity-100 hover:bg-[#70BFFF] hover:text-white",
            r#type: "button",
            onclick: move |_| {
              let is_correct = index == current_question.read().correct_answer;
              is_current_answer_correct.set(is_correct);
              
              if is_correct {
                correct_answers += 1;
              }

              let new_index = current_question_index + 1;

              current_question_index += 1;
              let new_question = questions.read();
              let new_question = new_question.get(new_index);

              if let Some(new_question) = new_question {
                current_question.set(new_question.clone());
                show_modal.set(true);
              }
              else {
                // TODO
              }
            },
            
            "{answer}"
          }
        }
      }
    }
  }
}
