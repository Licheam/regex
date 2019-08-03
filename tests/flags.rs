mat!(match_flag_case, "(?i)abc", "ABC", Some((0, 3)));
mat!(match_flag_weird_case, "(?i)a(?-i)bc", "Abc", Some((0, 3)));
mat!(match_flag_weird_case_not, "(?i)a(?-i)bc", "ABC", None);
mat!(match_flag_case_dotnl, "(?is)a.", "A\n", Some((0, 2)));
mat!(match_flag_case_dotnl_toggle, "(?is)a.(?-is)a.", "A\nab", Some((0, 4)));
mat!(match_flag_case_dotnl_toggle_not, "(?is)a.(?-is)a.", "A\na\n", None);
mat!(
    match_flag_case_dotnl_toggle_ok,
    "(?is)a.(?-is:a.)?",
    "A\na\n",
    Some((0, 2))
);
mat!(match_flag_multi, "(?m)(?:^\\d+$\n?)+", "123\n456\n789", Some((0, 11)));
mat!(match_flag_ungreedy, "(?U)a+", "aa", Some((0, 1)));
mat!(match_flag_ungreedy_greedy, "(?U)a+?", "aa", Some((0, 2)));
mat!(match_flag_ungreedy_noop, "(?U)(?-U)a+", "aa", Some((0, 2)));
