open Rsdd

let () =
  let robdd_builder = mk_bdd_builder_default_order 6L in
  let bdd =
    bdd_builder_compile_cnf robdd_builder
      (cnf_from_dimacs "\n    p cnf 6 3\n1 2 3 4 0\n-2 -3 4 5 0\n-4 -5 6 6 0\n") in
  let model_count = bdd_model_count robdd_builder bdd in
  print_endline (Int64.to_string model_count)

let () = 
  let builder = mk_bdd_builder_default_order 6L in 
  let bdd = bdd_exactlyone builder [0L; 1L; 2L ; 3L; 4L; 5L] in
  let mc = bdd_model_count builder bdd in 
  print_endline (Int64.to_string mc)

let () = 
  let builder = mk_bdd_builder_default_order 0L in 
  let (_, flip_one_half) = bdd_new_var builder true in 
  let (lbl1, decision_one) = bdd_new_var builder true in
  let (lbl2, decision_two) = bdd_new_var builder true in
  let (_, reward_5) = bdd_new_var builder true in
  let (_, reward_4) = bdd_new_var builder true in
  let (_, reward_0) = bdd_new_var builder true in
  let ite = bdd_ite builder flip_one_half reward_5 reward_0 in 
  let d1 = bdd_and builder decision_one ite in
  let d2 = bdd_and builder decision_two reward_4 in
  let xor = bdd_and builder (bdd_or builder decision_one decision_two) (bdd_negate builder (bdd_and builder decision_one decision_two)) in
  let total = bdd_and builder (bdd_or builder d1 d2) xor in 
  let sdfsdf = List.map mk_varlabel [lbl1; lbl2] in
  let param = new_wmc_params_eu [((0.5, 0.0), (0.5, 0.0));
                                 ((1.0, 0.0), (1.0, 0.0));
                                 ((1.0, 0.0), (1.0, 0.0));
                                 ((1.0, 0.0), (1.0, 5.0));
                                 ((1.0, 0.0), (1.0, 4.0));
                                 ((1.0, 0.0), (1.0, 0.0));] in
  let (asdf, _) = bdd_meu total (bdd_true builder) sdfsdf 3L param in
  let (a,b) = (extract asdf) in
  let asdfasdfasdf = String.concat " " [(Float.to_string a);(Float.to_string b)] in 
  print_endline(asdfasdfasdf)