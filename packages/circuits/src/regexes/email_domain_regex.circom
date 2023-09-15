pragma circom 2.1.5;
include "@zk-email/circuits/regexes/regex_helpers.circom";

template EmailDomainRegex(msg_bytes) {
	signal input msg[msg_bytes];
	signal output out;

	var num_bytes = msg_bytes+1;
	signal in[num_bytes];
	in[0]<==128;
	for (var i = 0; i < msg_bytes; i++) {
		in[i+1] <== msg[i];
	}

	component eq[36][num_bytes];
	component lt[24][num_bytes];
	component and[17][num_bytes];
	component multi_or[7][num_bytes];
	signal states[num_bytes+1][4];
	component state_changed[num_bytes];

	states[0][0] <== 1;
	for (var i = 1; i < 4; i++) {
		states[0][i] <== 0;
	}

	for (var i = 0; i < num_bytes; i++) {
		state_changed[i] = MultiOR(3);
		lt[0][i] = LessThan(8);
		lt[0][i].in[0] <== 47;
		lt[0][i].in[1] <== in[i];
		lt[1][i] = LessThan(8);
		lt[1][i].in[0] <== in[i];
		lt[1][i].in[1] <== 58;
		and[0][i] = AND();
		and[0][i].a <== lt[0][i].out;
		and[0][i].b <== lt[1][i].out;
		lt[2][i] = LessThan(8);
		lt[2][i].in[0] <== 64;
		lt[2][i].in[1] <== in[i];
		lt[3][i] = LessThan(8);
		lt[3][i].in[0] <== in[i];
		lt[3][i].in[1] <== 91;
		and[1][i] = AND();
		and[1][i].a <== lt[2][i].out;
		and[1][i].b <== lt[3][i].out;
		lt[4][i] = LessThan(8);
		lt[4][i].in[0] <== 96;
		lt[4][i].in[1] <== in[i];
		lt[5][i] = LessThan(8);
		lt[5][i].in[0] <== in[i];
		lt[5][i].in[1] <== 123;
		and[2][i] = AND();
		and[2][i].a <== lt[4][i].out;
		and[2][i].b <== lt[5][i].out;
		eq[0][i] = IsEqual();
		eq[0][i].in[0] <== in[i];
		eq[0][i].in[1] <== 91;
		eq[1][i] = IsEqual();
		eq[1][i].in[0] <== in[i];
		eq[1][i].in[1] <== 34;
		eq[2][i] = IsEqual();
		eq[2][i].in[0] <== in[i];
		eq[2][i].in[1] <== 37;
		eq[3][i] = IsEqual();
		eq[3][i].in[0] <== in[i];
		eq[3][i].in[1] <== 44;
		eq[4][i] = IsEqual();
		eq[4][i].in[0] <== in[i];
		eq[4][i].in[1] <== 43;
		eq[5][i] = IsEqual();
		eq[5][i].in[0] <== in[i];
		eq[5][i].in[1] <== 45;
		eq[6][i] = IsEqual();
		eq[6][i].in[0] <== in[i];
		eq[6][i].in[1] <== 46;
		eq[7][i] = IsEqual();
		eq[7][i].in[0] <== in[i];
		eq[7][i].in[1] <== 61;
		eq[8][i] = IsEqual();
		eq[8][i].in[0] <== in[i];
		eq[8][i].in[1] <== 95;
		eq[9][i] = IsEqual();
		eq[9][i].in[0] <== in[i];
		eq[9][i].in[1] <== 93;
		and[3][i] = AND();
		and[3][i].a <== states[i][0];
		multi_or[0][i] = MultiOR(13);
		multi_or[0][i].in[0] <== and[0][i].out;
		multi_or[0][i].in[1] <== and[1][i].out;
		multi_or[0][i].in[2] <== and[2][i].out;
		multi_or[0][i].in[3] <== eq[0][i].out;
		multi_or[0][i].in[4] <== eq[1][i].out;
		multi_or[0][i].in[5] <== eq[2][i].out;
		multi_or[0][i].in[6] <== eq[3][i].out;
		multi_or[0][i].in[7] <== eq[4][i].out;
		multi_or[0][i].in[8] <== eq[5][i].out;
		multi_or[0][i].in[9] <== eq[6][i].out;
		multi_or[0][i].in[10] <== eq[7][i].out;
		multi_or[0][i].in[11] <== eq[8][i].out;
		multi_or[0][i].in[12] <== eq[9][i].out;
		and[3][i].b <== multi_or[0][i].out;
		lt[6][i] = LessThan(8);
		lt[6][i].in[0] <== 47;
		lt[6][i].in[1] <== in[i];
		lt[7][i] = LessThan(8);
		lt[7][i].in[0] <== in[i];
		lt[7][i].in[1] <== 58;
		and[4][i] = AND();
		and[4][i].a <== lt[6][i].out;
		and[4][i].b <== lt[7][i].out;
		lt[8][i] = LessThan(8);
		lt[8][i].in[0] <== 64;
		lt[8][i].in[1] <== in[i];
		lt[9][i] = LessThan(8);
		lt[9][i].in[0] <== in[i];
		lt[9][i].in[1] <== 91;
		and[5][i] = AND();
		and[5][i].a <== lt[8][i].out;
		and[5][i].b <== lt[9][i].out;
		lt[10][i] = LessThan(8);
		lt[10][i].in[0] <== 96;
		lt[10][i].in[1] <== in[i];
		lt[11][i] = LessThan(8);
		lt[11][i].in[0] <== in[i];
		lt[11][i].in[1] <== 123;
		and[6][i] = AND();
		and[6][i].a <== lt[10][i].out;
		and[6][i].b <== lt[11][i].out;
		eq[10][i] = IsEqual();
		eq[10][i].in[0] <== in[i];
		eq[10][i].in[1] <== 91;
		eq[11][i] = IsEqual();
		eq[11][i].in[0] <== in[i];
		eq[11][i].in[1] <== 34;
		eq[12][i] = IsEqual();
		eq[12][i].in[0] <== in[i];
		eq[12][i].in[1] <== 37;
		eq[13][i] = IsEqual();
		eq[13][i].in[0] <== in[i];
		eq[13][i].in[1] <== 44;
		eq[14][i] = IsEqual();
		eq[14][i].in[0] <== in[i];
		eq[14][i].in[1] <== 43;
		eq[15][i] = IsEqual();
		eq[15][i].in[0] <== in[i];
		eq[15][i].in[1] <== 45;
		eq[16][i] = IsEqual();
		eq[16][i].in[0] <== in[i];
		eq[16][i].in[1] <== 46;
		eq[17][i] = IsEqual();
		eq[17][i].in[0] <== in[i];
		eq[17][i].in[1] <== 61;
		eq[18][i] = IsEqual();
		eq[18][i].in[0] <== in[i];
		eq[18][i].in[1] <== 95;
		eq[19][i] = IsEqual();
		eq[19][i].in[0] <== in[i];
		eq[19][i].in[1] <== 93;
		and[7][i] = AND();
		and[7][i].a <== states[i][1];
		multi_or[1][i] = MultiOR(13);
		multi_or[1][i].in[0] <== and[4][i].out;
		multi_or[1][i].in[1] <== and[5][i].out;
		multi_or[1][i].in[2] <== and[6][i].out;
		multi_or[1][i].in[3] <== eq[10][i].out;
		multi_or[1][i].in[4] <== eq[11][i].out;
		multi_or[1][i].in[5] <== eq[12][i].out;
		multi_or[1][i].in[6] <== eq[13][i].out;
		multi_or[1][i].in[7] <== eq[14][i].out;
		multi_or[1][i].in[8] <== eq[15][i].out;
		multi_or[1][i].in[9] <== eq[16][i].out;
		multi_or[1][i].in[10] <== eq[17][i].out;
		multi_or[1][i].in[11] <== eq[18][i].out;
		multi_or[1][i].in[12] <== eq[19][i].out;
		and[7][i].b <== multi_or[1][i].out;
		multi_or[2][i] = MultiOR(2);
		multi_or[2][i].in[0] <== and[3][i].out;
		multi_or[2][i].in[1] <== and[7][i].out;
		states[i+1][1] <== multi_or[2][i].out;
		state_changed[i].in[0] <== states[i+1][1];
		eq[20][i] = IsEqual();
		eq[20][i].in[0] <== in[i];
		eq[20][i].in[1] <== 91;
		eq[21][i] = IsEqual();
		eq[21][i].in[0] <== in[i];
		eq[21][i].in[1] <== 34;
		eq[22][i] = IsEqual();
		eq[22][i].in[0] <== in[i];
		eq[22][i].in[1] <== 64;
		eq[23][i] = IsEqual();
		eq[23][i].in[0] <== in[i];
		eq[23][i].in[1] <== 93;
		and[8][i] = AND();
		and[8][i].a <== states[i][1];
		multi_or[3][i] = MultiOR(4);
		multi_or[3][i].in[0] <== eq[20][i].out;
		multi_or[3][i].in[1] <== eq[21][i].out;
		multi_or[3][i].in[2] <== eq[22][i].out;
		multi_or[3][i].in[3] <== eq[23][i].out;
		and[8][i].b <== multi_or[3][i].out;
		states[i+1][2] <== and[8][i].out;
		state_changed[i].in[1] <== states[i+1][2];
		lt[12][i] = LessThan(8);
		lt[12][i].in[0] <== 47;
		lt[12][i].in[1] <== in[i];
		lt[13][i] = LessThan(8);
		lt[13][i].in[0] <== in[i];
		lt[13][i].in[1] <== 58;
		and[9][i] = AND();
		and[9][i].a <== lt[12][i].out;
		and[9][i].b <== lt[13][i].out;
		lt[14][i] = LessThan(8);
		lt[14][i].in[0] <== 64;
		lt[14][i].in[1] <== in[i];
		lt[15][i] = LessThan(8);
		lt[15][i].in[0] <== in[i];
		lt[15][i].in[1] <== 91;
		and[10][i] = AND();
		and[10][i].a <== lt[14][i].out;
		and[10][i].b <== lt[15][i].out;
		lt[16][i] = LessThan(8);
		lt[16][i].in[0] <== 96;
		lt[16][i].in[1] <== in[i];
		lt[17][i] = LessThan(8);
		lt[17][i].in[0] <== in[i];
		lt[17][i].in[1] <== 123;
		and[11][i] = AND();
		and[11][i].a <== lt[16][i].out;
		and[11][i].b <== lt[17][i].out;
		eq[24][i] = IsEqual();
		eq[24][i].in[0] <== in[i];
		eq[24][i].in[1] <== 91;
		eq[25][i] = IsEqual();
		eq[25][i].in[0] <== in[i];
		eq[25][i].in[1] <== 34;
		eq[26][i] = IsEqual();
		eq[26][i].in[0] <== in[i];
		eq[26][i].in[1] <== 45;
		eq[27][i] = IsEqual();
		eq[27][i].in[0] <== in[i];
		eq[27][i].in[1] <== 44;
		eq[28][i] = IsEqual();
		eq[28][i].in[0] <== in[i];
		eq[28][i].in[1] <== 46;
		eq[29][i] = IsEqual();
		eq[29][i].in[0] <== in[i];
		eq[29][i].in[1] <== 93;
		and[12][i] = AND();
		and[12][i].a <== states[i][2];
		multi_or[4][i] = MultiOR(9);
		multi_or[4][i].in[0] <== and[9][i].out;
		multi_or[4][i].in[1] <== and[10][i].out;
		multi_or[4][i].in[2] <== and[11][i].out;
		multi_or[4][i].in[3] <== eq[24][i].out;
		multi_or[4][i].in[4] <== eq[25][i].out;
		multi_or[4][i].in[5] <== eq[26][i].out;
		multi_or[4][i].in[6] <== eq[27][i].out;
		multi_or[4][i].in[7] <== eq[28][i].out;
		multi_or[4][i].in[8] <== eq[29][i].out;
		and[12][i].b <== multi_or[4][i].out;
		lt[18][i] = LessThan(8);
		lt[18][i].in[0] <== 47;
		lt[18][i].in[1] <== in[i];
		lt[19][i] = LessThan(8);
		lt[19][i].in[0] <== in[i];
		lt[19][i].in[1] <== 58;
		and[13][i] = AND();
		and[13][i].a <== lt[18][i].out;
		and[13][i].b <== lt[19][i].out;
		lt[20][i] = LessThan(8);
		lt[20][i].in[0] <== 64;
		lt[20][i].in[1] <== in[i];
		lt[21][i] = LessThan(8);
		lt[21][i].in[0] <== in[i];
		lt[21][i].in[1] <== 91;
		and[14][i] = AND();
		and[14][i].a <== lt[20][i].out;
		and[14][i].b <== lt[21][i].out;
		lt[22][i] = LessThan(8);
		lt[22][i].in[0] <== 96;
		lt[22][i].in[1] <== in[i];
		lt[23][i] = LessThan(8);
		lt[23][i].in[0] <== in[i];
		lt[23][i].in[1] <== 123;
		and[15][i] = AND();
		and[15][i].a <== lt[22][i].out;
		and[15][i].b <== lt[23][i].out;
		eq[30][i] = IsEqual();
		eq[30][i].in[0] <== in[i];
		eq[30][i].in[1] <== 91;
		eq[31][i] = IsEqual();
		eq[31][i].in[0] <== in[i];
		eq[31][i].in[1] <== 34;
		eq[32][i] = IsEqual();
		eq[32][i].in[0] <== in[i];
		eq[32][i].in[1] <== 45;
		eq[33][i] = IsEqual();
		eq[33][i].in[0] <== in[i];
		eq[33][i].in[1] <== 44;
		eq[34][i] = IsEqual();
		eq[34][i].in[0] <== in[i];
		eq[34][i].in[1] <== 46;
		eq[35][i] = IsEqual();
		eq[35][i].in[0] <== in[i];
		eq[35][i].in[1] <== 93;
		and[16][i] = AND();
		and[16][i].a <== states[i][3];
		multi_or[5][i] = MultiOR(9);
		multi_or[5][i].in[0] <== and[13][i].out;
		multi_or[5][i].in[1] <== and[14][i].out;
		multi_or[5][i].in[2] <== and[15][i].out;
		multi_or[5][i].in[3] <== eq[30][i].out;
		multi_or[5][i].in[4] <== eq[31][i].out;
		multi_or[5][i].in[5] <== eq[32][i].out;
		multi_or[5][i].in[6] <== eq[33][i].out;
		multi_or[5][i].in[7] <== eq[34][i].out;
		multi_or[5][i].in[8] <== eq[35][i].out;
		and[16][i].b <== multi_or[5][i].out;
		multi_or[6][i] = MultiOR(2);
		multi_or[6][i].in[0] <== and[12][i].out;
		multi_or[6][i].in[1] <== and[16][i].out;
		states[i+1][3] <== multi_or[6][i].out;
		state_changed[i].in[2] <== states[i+1][3];
		states[i+1][0] <== 1 - state_changed[i].out;
	}

	component final_state_result = MultiOR(num_bytes+1);
	for (var i = 0; i <= num_bytes; i++) {
		final_state_result.in[i] <== states[i][3];
	}
	out <== final_state_result.out;

	signal is_substr0[msg_bytes][3];
	signal output reveal0[msg_bytes];
	for (var i = 0; i < msg_bytes; i++) {
		is_substr0[i][0] <== 0;
		is_substr0[i][1] <== is_substr0[i][0] + states[i+1][3] * states[i+2][3];
		is_substr0[i][2] <== is_substr0[i][1] + states[i+1][2] * states[i+2][3];
		reveal0[i] <== in[i+1] * is_substr0[i][2];
	}
}