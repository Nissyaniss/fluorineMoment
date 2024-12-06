use dioxus::prelude::*;

#[derive(Clone)]
struct Question {
  context: String,
  question: String,
  answers: Vec<String>,
  correct_answer: usize,
  correct_details: String
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
      correct_details: "La peau agit comme un régulateur de température pour le corps humain, tout comme l'océan agit comme une mémoire du climat en captant la chaleur.".to_string(),
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
      correct_details: "Le système circulatoire transporte le sang, les nutriments et l'oxygène à travers le corps, tout comme les courants océaniques transportent de la chaleur et des nutriments à travers la planète.".to_string(),
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
      correct_details: "Les poumons sont responsables des échanges gazeux dans le corps humain, tout comme les échanges de gaz entre les profondeurs et la surface de l'océan.".to_string(),
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
      correct_details: "Les cellules sont la base de la vie et produisent de l'énergie pour le corps humain, tout comme le phytoplancton est la base de la chaîne alimentaire marine.".to_string(),
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
      correct_details: "Les graisses (tissu adipeux) stockent des déchets et des substances dans le corps humain, tout comme une partie du phytoplancton en décomposition est emprisonnée au fond de l'océan.".to_string(),
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
      correct_details: "Le sang est un équilibre chimique délicat dans le corps humain, tout comme l'acidification des océans modifie leur équilibre chimique et affecte les écosystèmes marins.".to_string(),
    },
  ]);

  let mut current_question_index = use_signal(|| 0);
  let mut current_question = use_signal(|| questions.read()[0].clone());
  let mut correct_answers = use_signal(|| 0);

  let mut is_current_answer_correct = use_signal(|| false);
  let mut show_modal = use_signal(|| false);
  let mut last_explanation = use_signal(|| "".to_string());

  rsx! {
    if *show_modal.read() {
      div {
        class: "z-50 fixed inset-0 bg-black bg-opacity-20 flex items-center justify-center",
        div {
          class: "bg-white max-w-[400px]",

          div {
            class: "h-[250px] w-full bg-[#F5F5F5]",
          }

          div {
            class: "p-6",

            if *is_current_answer_correct.read() {
              p {
                class: "text-[#37A5FF] text-xl",
                "Vrai !"
              }
            }
            else {
              p {
                class: "text-[#616161] text-xl",
                "Et non..."
              }
            }
  
            p {
              class: "font-[Arial] mb-8 mt-4",
              "{last_explanation}"
            }
          }


          button {
            class: "py-4 bg-[#37A5FF] hover:bg-[#4DAFFF] text-white w-full",
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
            class: "px-3 py-1.5 opacity-80 hover:opacity-100 hover:bg-[#37A5FF] hover:text-white text-left",
            r#type: "button",
            onclick: move |_| {
              let is_correct = index == current_question.read().correct_answer;
              let current_explanation = current_question.read().correct_details.clone();
              is_current_answer_correct.set(is_correct);
              
              if is_correct {
                correct_answers += 1;
              }

              let new_index = current_question_index + 1;

              current_question_index += 1;
              let new_question_ = questions.read();
              let new_question = new_question_.get(new_index);

              if let Some(new_question) = new_question {
                current_question.set(new_question.clone());
                show_modal.set(true);
                last_explanation.set(current_explanation);
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
