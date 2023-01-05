mod common;
use common::assert_output_file;

const MINIMAL_MATCH_STATEMENT_DOT_OUTPUT: &str = r#"digraph petrinet {
    PROGRAM_END [shape="circle" xlabel="PROGRAM_END" label=""];
    PROGRAM_PANIC [shape="circle" xlabel="PROGRAM_PANIC" label=""];
    PROGRAM_START [shape="circle" xlabel="PROGRAM_START" label="•"];
    main_BASIC_BLOCK_1 [shape="circle" xlabel="main_BASIC_BLOCK_1" label=""];
    main_BASIC_BLOCK_2 [shape="circle" xlabel="main_BASIC_BLOCK_2" label=""];
    main_BASIC_BLOCK_3 [shape="circle" xlabel="main_BASIC_BLOCK_3" label=""];
    main_BASIC_BLOCK_4 [shape="circle" xlabel="main_BASIC_BLOCK_4" label=""];
    main_BASIC_BLOCK_5 [shape="circle" xlabel="main_BASIC_BLOCK_5" label=""];
    main_BASIC_BLOCK_6 [shape="circle" xlabel="main_BASIC_BLOCK_6" label=""];
    main_BASIC_BLOCK_7 [shape="circle" xlabel="main_BASIC_BLOCK_7" label=""];
    main_BASIC_BLOCK_8 [shape="circle" xlabel="main_BASIC_BLOCK_8" label=""];
    main_BASIC_BLOCK_9 [shape="circle" xlabel="main_BASIC_BLOCK_9" label=""];
    main_BASIC_BLOCK_END_PLACE_1 [shape="circle" xlabel="main_BASIC_BLOCK_END_PLACE_1" label=""];
    main_BLOCK_1_STATEMENT_0 [shape="box" xlabel="main_BLOCK_1_STATEMENT_0" label=""];
    main_DROP_6 [shape="box" xlabel="main_DROP_6" label=""];
    main_DROP_8 [shape="box" xlabel="main_DROP_8" label=""];
    main_GOTO_3 [shape="box" xlabel="main_GOTO_3" label=""];
    main_GOTO_4 [shape="box" xlabel="main_GOTO_4" label=""];
    main_GOTO_5 [shape="box" xlabel="main_GOTO_5" label=""];
    main_RETURN [shape="box" xlabel="main_RETURN" label=""];
    main_SWITCH_INT_3 [shape="box" xlabel="main_SWITCH_INT_3" label=""];
    main_SWITCH_INT_4 [shape="box" xlabel="main_SWITCH_INT_4" label=""];
    main_SWITCH_INT_5 [shape="box" xlabel="main_SWITCH_INT_5" label=""];
    main_UNWIND_9 [shape="box" xlabel="main_UNWIND_9" label=""];
    std_env_args_FOREIGN_CALL [shape="box" xlabel="std_env_args_FOREIGN_CALL" label=""];
    std_iter_ExactSizeIterator_len_FOREIGN_CALL [shape="box" xlabel="std_iter_ExactSizeIterator_len_FOREIGN_CALL" label=""];
    PROGRAM_START -> std_env_args_FOREIGN_CALL;
    main_BASIC_BLOCK_1 -> main_BLOCK_1_STATEMENT_0;
    main_BASIC_BLOCK_2 -> main_SWITCH_INT_3;
    main_BASIC_BLOCK_2 -> main_SWITCH_INT_4;
    main_BASIC_BLOCK_2 -> main_SWITCH_INT_5;
    main_BASIC_BLOCK_3 -> main_GOTO_3;
    main_BASIC_BLOCK_4 -> main_GOTO_4;
    main_BASIC_BLOCK_5 -> main_GOTO_5;
    main_BASIC_BLOCK_6 -> main_DROP_6;
    main_BASIC_BLOCK_7 -> main_RETURN;
    main_BASIC_BLOCK_8 -> main_DROP_8;
    main_BASIC_BLOCK_9 -> main_UNWIND_9;
    main_BASIC_BLOCK_END_PLACE_1 -> std_iter_ExactSizeIterator_len_FOREIGN_CALL;
    main_BLOCK_1_STATEMENT_0 -> main_BASIC_BLOCK_END_PLACE_1;
    main_DROP_6 -> main_BASIC_BLOCK_7;
    main_DROP_8 -> main_BASIC_BLOCK_9;
    main_GOTO_3 -> main_BASIC_BLOCK_6;
    main_GOTO_4 -> main_BASIC_BLOCK_6;
    main_GOTO_5 -> main_BASIC_BLOCK_6;
    main_RETURN -> PROGRAM_END;
    main_SWITCH_INT_3 -> main_BASIC_BLOCK_3;
    main_SWITCH_INT_4 -> main_BASIC_BLOCK_4;
    main_SWITCH_INT_5 -> main_BASIC_BLOCK_5;
    main_UNWIND_9 -> PROGRAM_PANIC;
    std_env_args_FOREIGN_CALL -> main_BASIC_BLOCK_1;
    std_iter_ExactSizeIterator_len_FOREIGN_CALL -> main_BASIC_BLOCK_2;
    std_iter_ExactSizeIterator_len_FOREIGN_CALL -> main_BASIC_BLOCK_8;
}
"#;

const MINIMAL_MATCH_STATEMENT_LOLA_OUTPUT: &str = r#"PLACE
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START,
    main_BASIC_BLOCK_1,
    main_BASIC_BLOCK_2,
    main_BASIC_BLOCK_3,
    main_BASIC_BLOCK_4,
    main_BASIC_BLOCK_5,
    main_BASIC_BLOCK_6,
    main_BASIC_BLOCK_7,
    main_BASIC_BLOCK_8,
    main_BASIC_BLOCK_9,
    main_BASIC_BLOCK_END_PLACE_1;

MARKING
    PROGRAM_START : 1,
TRANSITION main_BLOCK_1_STATEMENT_0
  CONSUME
    main_BASIC_BLOCK_1 : 1;
  PRODUCE
    main_BASIC_BLOCK_END_PLACE_1 : 1;
TRANSITION main_DROP_6
  CONSUME
    main_BASIC_BLOCK_6 : 1;
  PRODUCE
    main_BASIC_BLOCK_7 : 1;
TRANSITION main_DROP_8
  CONSUME
    main_BASIC_BLOCK_8 : 1;
  PRODUCE
    main_BASIC_BLOCK_9 : 1;
TRANSITION main_GOTO_3
  CONSUME
    main_BASIC_BLOCK_3 : 1;
  PRODUCE
    main_BASIC_BLOCK_6 : 1;
TRANSITION main_GOTO_4
  CONSUME
    main_BASIC_BLOCK_4 : 1;
  PRODUCE
    main_BASIC_BLOCK_6 : 1;
TRANSITION main_GOTO_5
  CONSUME
    main_BASIC_BLOCK_5 : 1;
  PRODUCE
    main_BASIC_BLOCK_6 : 1;
TRANSITION main_RETURN
  CONSUME
    main_BASIC_BLOCK_7 : 1;
  PRODUCE
    PROGRAM_END : 1;
TRANSITION main_SWITCH_INT_3
  CONSUME
    main_BASIC_BLOCK_2 : 1;
  PRODUCE
    main_BASIC_BLOCK_3 : 1;
TRANSITION main_SWITCH_INT_4
  CONSUME
    main_BASIC_BLOCK_2 : 1;
  PRODUCE
    main_BASIC_BLOCK_4 : 1;
TRANSITION main_SWITCH_INT_5
  CONSUME
    main_BASIC_BLOCK_2 : 1;
  PRODUCE
    main_BASIC_BLOCK_5 : 1;
TRANSITION main_UNWIND_9
  CONSUME
    main_BASIC_BLOCK_9 : 1;
  PRODUCE
    PROGRAM_PANIC : 1;
TRANSITION std_env_args_FOREIGN_CALL
  CONSUME
    PROGRAM_START : 1;
  PRODUCE
    main_BASIC_BLOCK_1 : 1;
TRANSITION std_iter_ExactSizeIterator_len_FOREIGN_CALL
  CONSUME
    main_BASIC_BLOCK_END_PLACE_1 : 1;
  PRODUCE
    main_BASIC_BLOCK_2 : 1,
    main_BASIC_BLOCK_8 : 1;
"#;

const MINIMAL_MATCH_STATEMENT_PNML_OUTPUT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<pnml xmlns="http://www.pnml.org/version-2009/grammar/pnml">
  <net id="net0" type="http://www.pnml.org/version-2009/grammar/ptnet">
    <page id="page0">
      <place id="PROGRAM_END">
        <name>
          <text>PROGRAM_END</text>
        </name>
      </place>
      <place id="PROGRAM_PANIC">
        <name>
          <text>PROGRAM_PANIC</text>
        </name>
      </place>
      <place id="PROGRAM_START">
        <name>
          <text>PROGRAM_START</text>
        </name>
        <initialMarking>
          <text>1</text>
        </initialMarking>
      </place>
      <place id="main_BASIC_BLOCK_1">
        <name>
          <text>main_BASIC_BLOCK_1</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_2">
        <name>
          <text>main_BASIC_BLOCK_2</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_3">
        <name>
          <text>main_BASIC_BLOCK_3</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_4">
        <name>
          <text>main_BASIC_BLOCK_4</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_5">
        <name>
          <text>main_BASIC_BLOCK_5</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_6">
        <name>
          <text>main_BASIC_BLOCK_6</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_7">
        <name>
          <text>main_BASIC_BLOCK_7</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_8">
        <name>
          <text>main_BASIC_BLOCK_8</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_9">
        <name>
          <text>main_BASIC_BLOCK_9</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_END_PLACE_1">
        <name>
          <text>main_BASIC_BLOCK_END_PLACE_1</text>
        </name>
      </place>
      <transition id="main_BLOCK_1_STATEMENT_0">
        <name>
          <text>main_BLOCK_1_STATEMENT_0</text>
        </name>
      </transition>
      <transition id="main_DROP_6">
        <name>
          <text>main_DROP_6</text>
        </name>
      </transition>
      <transition id="main_DROP_8">
        <name>
          <text>main_DROP_8</text>
        </name>
      </transition>
      <transition id="main_GOTO_3">
        <name>
          <text>main_GOTO_3</text>
        </name>
      </transition>
      <transition id="main_GOTO_4">
        <name>
          <text>main_GOTO_4</text>
        </name>
      </transition>
      <transition id="main_GOTO_5">
        <name>
          <text>main_GOTO_5</text>
        </name>
      </transition>
      <transition id="main_RETURN">
        <name>
          <text>main_RETURN</text>
        </name>
      </transition>
      <transition id="main_SWITCH_INT_3">
        <name>
          <text>main_SWITCH_INT_3</text>
        </name>
      </transition>
      <transition id="main_SWITCH_INT_4">
        <name>
          <text>main_SWITCH_INT_4</text>
        </name>
      </transition>
      <transition id="main_SWITCH_INT_5">
        <name>
          <text>main_SWITCH_INT_5</text>
        </name>
      </transition>
      <transition id="main_UNWIND_9">
        <name>
          <text>main_UNWIND_9</text>
        </name>
      </transition>
      <transition id="std_env_args_FOREIGN_CALL">
        <name>
          <text>std_env_args_FOREIGN_CALL</text>
        </name>
      </transition>
      <transition id="std_iter_ExactSizeIterator_len_FOREIGN_CALL">
        <name>
          <text>std_iter_ExactSizeIterator_len_FOREIGN_CALL</text>
        </name>
      </transition>
      <arc source="PROGRAM_START" target="std_env_args_FOREIGN_CALL" id="(PROGRAM_START, std_env_args_FOREIGN_CALL)">
        <name>
          <text>(PROGRAM_START, std_env_args_FOREIGN_CALL)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_1" target="main_BLOCK_1_STATEMENT_0" id="(main_BASIC_BLOCK_1, main_BLOCK_1_STATEMENT_0)">
        <name>
          <text>(main_BASIC_BLOCK_1, main_BLOCK_1_STATEMENT_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_2" target="main_SWITCH_INT_3" id="(main_BASIC_BLOCK_2, main_SWITCH_INT_3)">
        <name>
          <text>(main_BASIC_BLOCK_2, main_SWITCH_INT_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_2" target="main_SWITCH_INT_4" id="(main_BASIC_BLOCK_2, main_SWITCH_INT_4)">
        <name>
          <text>(main_BASIC_BLOCK_2, main_SWITCH_INT_4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_2" target="main_SWITCH_INT_5" id="(main_BASIC_BLOCK_2, main_SWITCH_INT_5)">
        <name>
          <text>(main_BASIC_BLOCK_2, main_SWITCH_INT_5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_3" target="main_GOTO_3" id="(main_BASIC_BLOCK_3, main_GOTO_3)">
        <name>
          <text>(main_BASIC_BLOCK_3, main_GOTO_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_4" target="main_GOTO_4" id="(main_BASIC_BLOCK_4, main_GOTO_4)">
        <name>
          <text>(main_BASIC_BLOCK_4, main_GOTO_4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_5" target="main_GOTO_5" id="(main_BASIC_BLOCK_5, main_GOTO_5)">
        <name>
          <text>(main_BASIC_BLOCK_5, main_GOTO_5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_6" target="main_DROP_6" id="(main_BASIC_BLOCK_6, main_DROP_6)">
        <name>
          <text>(main_BASIC_BLOCK_6, main_DROP_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_7" target="main_RETURN" id="(main_BASIC_BLOCK_7, main_RETURN)">
        <name>
          <text>(main_BASIC_BLOCK_7, main_RETURN)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_8" target="main_DROP_8" id="(main_BASIC_BLOCK_8, main_DROP_8)">
        <name>
          <text>(main_BASIC_BLOCK_8, main_DROP_8)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_9" target="main_UNWIND_9" id="(main_BASIC_BLOCK_9, main_UNWIND_9)">
        <name>
          <text>(main_BASIC_BLOCK_9, main_UNWIND_9)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_END_PLACE_1" target="std_iter_ExactSizeIterator_len_FOREIGN_CALL" id="(main_BASIC_BLOCK_END_PLACE_1, std_iter_ExactSizeIterator_len_FOREIGN_CALL)">
        <name>
          <text>(main_BASIC_BLOCK_END_PLACE_1, std_iter_ExactSizeIterator_len_FOREIGN_CALL)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BLOCK_1_STATEMENT_0" target="main_BASIC_BLOCK_END_PLACE_1" id="(main_BLOCK_1_STATEMENT_0, main_BASIC_BLOCK_END_PLACE_1)">
        <name>
          <text>(main_BLOCK_1_STATEMENT_0, main_BASIC_BLOCK_END_PLACE_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_6" target="main_BASIC_BLOCK_7" id="(main_DROP_6, main_BASIC_BLOCK_7)">
        <name>
          <text>(main_DROP_6, main_BASIC_BLOCK_7)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_8" target="main_BASIC_BLOCK_9" id="(main_DROP_8, main_BASIC_BLOCK_9)">
        <name>
          <text>(main_DROP_8, main_BASIC_BLOCK_9)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_GOTO_3" target="main_BASIC_BLOCK_6" id="(main_GOTO_3, main_BASIC_BLOCK_6)">
        <name>
          <text>(main_GOTO_3, main_BASIC_BLOCK_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_GOTO_4" target="main_BASIC_BLOCK_6" id="(main_GOTO_4, main_BASIC_BLOCK_6)">
        <name>
          <text>(main_GOTO_4, main_BASIC_BLOCK_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_GOTO_5" target="main_BASIC_BLOCK_6" id="(main_GOTO_5, main_BASIC_BLOCK_6)">
        <name>
          <text>(main_GOTO_5, main_BASIC_BLOCK_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_RETURN" target="PROGRAM_END" id="(main_RETURN, PROGRAM_END)">
        <name>
          <text>(main_RETURN, PROGRAM_END)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_SWITCH_INT_3" target="main_BASIC_BLOCK_3" id="(main_SWITCH_INT_3, main_BASIC_BLOCK_3)">
        <name>
          <text>(main_SWITCH_INT_3, main_BASIC_BLOCK_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_SWITCH_INT_4" target="main_BASIC_BLOCK_4" id="(main_SWITCH_INT_4, main_BASIC_BLOCK_4)">
        <name>
          <text>(main_SWITCH_INT_4, main_BASIC_BLOCK_4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_SWITCH_INT_5" target="main_BASIC_BLOCK_5" id="(main_SWITCH_INT_5, main_BASIC_BLOCK_5)">
        <name>
          <text>(main_SWITCH_INT_5, main_BASIC_BLOCK_5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_UNWIND_9" target="PROGRAM_PANIC" id="(main_UNWIND_9, PROGRAM_PANIC)">
        <name>
          <text>(main_UNWIND_9, PROGRAM_PANIC)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_env_args_FOREIGN_CALL" target="main_BASIC_BLOCK_1" id="(std_env_args_FOREIGN_CALL, main_BASIC_BLOCK_1)">
        <name>
          <text>(std_env_args_FOREIGN_CALL, main_BASIC_BLOCK_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_iter_ExactSizeIterator_len_FOREIGN_CALL" target="main_BASIC_BLOCK_2" id="(std_iter_ExactSizeIterator_len_FOREIGN_CALL, main_BASIC_BLOCK_2)">
        <name>
          <text>(std_iter_ExactSizeIterator_len_FOREIGN_CALL, main_BASIC_BLOCK_2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_iter_ExactSizeIterator_len_FOREIGN_CALL" target="main_BASIC_BLOCK_8" id="(std_iter_ExactSizeIterator_len_FOREIGN_CALL, main_BASIC_BLOCK_8)">
        <name>
          <text>(std_iter_ExactSizeIterator_len_FOREIGN_CALL, main_BASIC_BLOCK_8)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;

#[test]
fn minimal_match_statement_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_match_statement.rs",
        "dot",
        "./net.dot",
        MINIMAL_MATCH_STATEMENT_DOT_OUTPUT,
    );
}

#[test]
fn minimal_match_statement_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_match_statement.rs",
        "lola",
        "./net.lola",
        MINIMAL_MATCH_STATEMENT_LOLA_OUTPUT,
    );
}

#[test]
fn minimal_match_statement_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_match_statement.rs",
        "pnml",
        "./net.pnml",
        MINIMAL_MATCH_STATEMENT_PNML_OUTPUT,
    );
}
