use crate::Graph;
use crate::merge::*;
use crate::utils::{into_v16, from_v16};

fn graph_samples() -> Vec<(Graph, String)> {
    let mut result = vec![];
    let mut linear = Graph::new(32, 32)
        .set_title("test".to_string())
        .set_y_axis_label("num".to_string())
        .set_x_axis_label("ber".to_string())
        .set_y_max(278)
        .set_y_min(2)
        .set_1d_data(
            (0..256).map(|i| (i.to_string(), i)).collect()
        ).to_owned();
    let linear_answer1 =
"                  test                  
      num                               
   278 |                                
       |                                
       |                                
   254 |                               ▄
       |                              ▄█
       |                             ▄██
   230 |                            ▄███
       |                           ▄████
       |                          ▄█████
   206 |                         ▄██████
       |                        ▄███████
       |                       ▄████████
   182 |                      ▄█████████
       |                     ▄██████████
       |                    ▄███████████
   158 |                   ▄████████████
       |                  ▄█████████████
       |                 ▄██████████████
   134 |                ▄███████████████
       |               ▄████████████████
       |              ▄█████████████████
   110 |             ▄██████████████████
       |            ▄███████████████████
       |           ▄████████████████████
    86 |          ▄█████████████████████
       |         ▄██████████████████████
       |        ▄███████████████████████
    62 |       ▄████████████████████████
       |      ▄█████████████████████████
       |     ▄██████████████████████████
    38 |    ▄███████████████████████████
       |   ▄████████████████████████████
       |--------------------------------
        0       64      128     192     
                                     ber
".to_string();

    result.push((linear.clone(), linear_answer1));

    linear.set_plot_height(12);
    linear.set_plot_width(12);
    linear.set_x_label_interval(4);

    let linear_answer2 =
"        test        
      num           
   278 |            
       |           ▄
       |          ▄█
   209 |         ▄██
       |        ▄███
       |       ▄████
   140 |      ▄█████
       |     ███████
       |    ████████
    71 |   █████████
       |  ██████████
       | ███████████
     2 |------------
        0   85  170 
                 ber
".to_string();

    result.push((linear.clone(), linear_answer2));

    linear.set_paddings([1, 2, 3, 4]);

    let linear_answer3 =
"                           
           test            
         num               
      278 |                
          |           ▄    
          |          ▄█    
      209 |         ▄██    
          |        ▄███    
          |       ▄████    
      140 |      ▄█████    
          |     ███████    
          |    ████████    
       71 |   █████████    
          |  ██████████    
          | ███████████    
        2 |------------    
           0   85  170     
                    ber    
                           
                           
".to_string();
    result.push((linear.clone(), linear_answer3));

    linear.set_paddings([0, 0, 0, 0]);
    linear.set_plot_width(72);
    let linear_answer4 =
"                                      test                                      
      num                                                                       
   278 |                                                                        
       |                                                                  ▄▄▄███
       |                                                            ▄▄▄█████████
   209 |                                                     ▄▄▄████████████████
       |                                               ▄▄▄██████████████████████
       |                                        ▄▄▄█████████████████████████████
   140 |                                  ▄▄▄███████████████████████████████████
       |                           ▄▄▄██████████████████████████████████████████
       |                     ▄▄▄████████████████████████████████████████████████
    71 |              ▄▄▄███████████████████████████████████████████████████████
       |        ▄▄▄█████████████████████████████████████████████████████████████
       | ▄▄▄████████████████████████████████████████████████████████████████████
     2 |------------------------------------------------------------------------
        0   14  28  42  56  71  85  99  113 128 142 156 170 184 199 213 227 241 
                                                                             ber
".to_string();

    result.push((linear.clone(), linear_answer4));

    linear.set_plot_width(12).set_y_axis_label("숫자".to_string());

    let linear_answer5 =
"        test        
      숫자          
   278 |            
       |           ▄
       |          ▄█
   209 |         ▄██
       |        ▄███
       |       ▄████
   140 |      ▄█████
       |     ███████
       |    ████████
    71 |   █████████
       |  ██████████
       | ███████████
     2 |------------
        0   85  170 
                 ber
".to_string();

    result.push((linear.clone(), linear_answer5));

    result
}

#[test]
fn test_graph_samples() {
    let mut failures = vec![];

    for (graph, answer) in graph_samples().into_iter() {
        let result = graph.draw();

        if result != answer {
            failures.push(find_difference(&result, &answer));
        }

    }

    if failures.len() > 0 {
        panic!("{} out of {} cases have failed!!!\n\n{}", failures.len(), graph_samples().len(), failures.concat());
    }

}

#[test]
fn merge_test() {
    let sqr1 =
"****
****
****
****".to_string();

    let sqr2 =
"******
******
******
******
******
******".to_string();

    let mut test_cases = vec![];

    test_cases.push(
        (merge_horiz(&sqr1, &sqr1, 0),
"********
********
********
********".to_string())
    );

    test_cases.push(
        (merge_horiz(&sqr1, &sqr1, 2),
"****  ****
****  ****
****  ****
****  ****".to_string())
    );

    test_cases.push((merge_vert(&sqr1, &sqr1, 0, Alignment::Left),
"****
****
****
****
****
****
****
****".to_string()));

    test_cases.push((merge_vert(&sqr1, &sqr1, 2, Alignment::Left),
"****
****
****
****
    
    
****
****
****
****".to_string()));

    test_cases.push((merge_vert(&sqr1, &sqr2, 0, Alignment::Left),
"****  
****  
****  
****  
******
******
******
******
******
******".to_string()));

    test_cases.push((merge_vert(&sqr2, &sqr1, 0, Alignment::Left),
"******
******
******
******
******
******
****  
****  
****  
****  ".to_string()));

    test_cases.push((merge_vert(&sqr1, &sqr2, 2, Alignment::Left),
"****  
****  
****  
****  
      
      
******
******
******
******
******
******".to_string()));

    test_cases.push((merge_vert(&sqr2, &sqr1, 2, Alignment::Left),
"******
******
******
******
******
******
      
      
****  
****  
****  
****  ".to_string()));

    test_cases.push((merge_vert(&sqr2, &sqr1, 0, Alignment::Center),
"******
******
******
******
******
******
 **** 
 **** 
 **** 
 **** ".to_string()));

    test_cases.push((merge_vert(&sqr2, &sqr1, 0, Alignment::Right),
"******
******
******
******
******
******
  ****
  ****
  ****
  ****".to_string()));

    test_cases.push((merge_vert(&sqr1, &sqr2, 0, Alignment::Center),
" **** 
 **** 
 **** 
 **** 
******
******
******
******
******
******".to_string()));

    test_cases.push((merge_vert(&sqr1, &sqr2, 0, Alignment::Right),
"  ****
  ****
  ****
  ****
******
******
******
******
******
******".to_string()));

    let mut failures = Vec::with_capacity(test_cases.len());

    for (result, answer) in test_cases.iter() {

        if result != answer {
            failures.push(find_difference(&result, &answer));
        }

    }

    if failures.len() > 0 {
        panic!("{} out of {} cases have failed!!!\n\n{}", failures.len(), test_cases.len(), failures.concat());
    }

}

fn find_difference(result: &String, answer: &String) -> String {
    let lines_r = into_v16(result).split(|c| *c == '\n' as u16).map(|line| line.to_vec()).collect::<Vec<Vec<u16>>>();
    let lines_a = into_v16(answer).split(|c| *c == '\n' as u16).map(|line| line.to_vec()).collect::<Vec<Vec<u16>>>();
    let mut diffs = vec![];

    for i in 0..(lines_r.len().min(lines_a.len())) {

        if lines_r[i] != lines_a[i] {
            diffs.push(format!("{i}: {:?}, {:?}\n", from_v16(&lines_r[i]), from_v16(&lines_a[i])));
        }

    }

    let diffs = diffs.concat();

    format!(
        "\n\nresult\n\n{result}\n\nanswer\n\n{answer}\n\ndiffs\n\n{diffs}\n\n"
    )
}