// Solve problem 13. Large Sum
//
// Work out the first ten digits of the sum of the following one-hundred 50-digit numbers
//
// 37107287533902102798797998220837590246510135740250
// 46376937677490009712648124896970078050417018260538
// 74324986199524741059474233309513058123726617309629
// 91942213363574161572522430563301811072406154908250
// 23067588207539346171171980310421047513778063246676
// 89261670696623633820136378418383684178734361726757
// 28112879812849979408065481931592621691275889832738
// 44274228917432520321923589422876796487670272189318
// 47451445736001306439091167216856844588711603153276
// 70386486105843025439939619828917593665686757934951
// 62176457141856560629502157223196586755079324193331
// 64906352462741904929101432445813822663347944758178
// 92575867718337217661963751590579239728245598838407
// 58203565325359399008402633568948830189458628227828
// 80181199384826282014278194139940567587151170094390
// 35398664372827112653829987240784473053190104293586
// 86515506006295864861532075273371959191420517255829
// 71693888707715466499115593487603532921714970056938
// 54370070576826684624621495650076471787294438377604
// 53282654108756828443191190634694037855217779295145
// 36123272525000296071075082563815656710885258350721
// 45876576172410976447339110607218265236877223636045
// 17423706905851860660448207621209813287860733969412
// 81142660418086830619328460811191061556940512689692
// 51934325451728388641918047049293215058642563049483
// 62467221648435076201727918039944693004732956340691
// 15732444386908125794514089057706229429197107928209
// 55037687525678773091862540744969844508330393682126
// 18336384825330154686196124348767681297534375946515
// 80386287592878490201521685554828717201219257766954
// 78182833757993103614740356856449095527097864797581
// 16726320100436897842553539920931837441497806860984
// 48403098129077791799088218795327364475675590848030
// 87086987551392711854517078544161852424320693150332
// 59959406895756536782107074926966537676326235447210
// 69793950679652694742597709739166693763042633987085
// 41052684708299085211399427365734116182760315001271
// 65378607361501080857009149939512557028198746004375
// 35829035317434717326932123578154982629742552737307
// 94953759765105305946966067683156574377167401875275
// 88902802571733229619176668713819931811048770190271
// 25267680276078003013678680992525463401061632866526
// 36270218540497705585629946580636237993140746255962
// 24074486908231174977792365466257246923322810917141
// 91430288197103288597806669760892938638285025333403
// 34413065578016127815921815005561868836468420090470
// 23053081172816430487623791969842487255036638784583
// 11487696932154902810424020138335124462181441773470
// 63783299490636259666498587618221225225512486764533
// 67720186971698544312419572409913959008952310058822
// 95548255300263520781532296796249481641953868218774
// 76085327132285723110424803456124867697064507995236
// 37774242535411291684276865538926205024910326572967
// 23701913275725675285653248258265463092207058596522
// 29798860272258331913126375147341994889534765745501
// 18495701454879288984856827726077713721403798879715
// 38298203783031473527721580348144513491373226651381
// 34829543829199918180278916522431027392251122869539
// 40957953066405232632538044100059654939159879593635
// 29746152185502371307642255121183693803580388584903
// 41698116222072977186158236678424689157993532961922
// 62467957194401269043877107275048102390895523597457
// 23189706772547915061505504953922979530901129967519
// 86188088225875314529584099251203829009407770775672
// 11306739708304724483816533873502340845647058077308
// 82959174767140363198008187129011875491310547126581
// 97623331044818386269515456334926366572897563400500
// 42846280183517070527831839425882145521227251250327
// 55121603546981200581762165212827652751691296897789
// 32238195734329339946437501907836945765883352399886
// 75506164965184775180738168837861091527357929701337
// 62177842752192623401942399639168044983993173312731
// 32924185707147349566916674687634660915035914677504
// 99518671430235219628894890102423325116913619626622
// 73267460800591547471830798392868535206946944540724
// 76841822524674417161514036427982273348055556214818
// 97142617910342598647204516893989422179826088076852
// 87783646182799346313767754307809363333018982642090
// 10848802521674670883215120185883543223812876952786
// 71329612474782464538636993009049310363619763878039
// 62184073572399794223406235393808339651327408011116
// 66627891981488087797941876876144230030984490851411
// 60661826293682836764744779239180335110989069790714
// 85786944089552990653640447425576083659976645795096
// 66024396409905389607120198219976047599490197230297
// 64913982680032973156037120041377903785566085089252
// 16730939319872750275468906903707539413042652315011
// 94809377245048795150954100921645863754710598436791
// 78639167021187492431995700641917969777599028300699
// 15368713711936614952811305876380278410754449733078
// 40789923115535562561142322423255033685442488917353
// 44889911501440648020369068063960672322193204149535
// 41503128880339536053299340368006977710650566631954
// 81234880673210146739058568557934581403627822703280
// 82616570773948327592232845941706525094512325230608
// 22918802058777319719839450180888072429661980811197
// 77158542502016545090413245809786882778948721859617
// 72107838435069186155435662884062257473692284509516
// 20849603980134001723930671666823555245252804609722
// 53503534226472524250874054075591789781264330331690
// 

// The following function is provided because I couldn't
// get the builtin .to_digit function to compile.
// At some point this should be changed over to the builtin
// to_digit method.

const NUMBER_SIZE: usize = 50;
const SET_SIZE: usize = 100;
const BASE10: u32 = 10;

fn char2digit(item: char) -> u32 {
//    match item {
//        '0' => return 0,
//        '1' => return 1,
//        '2' => return 2,
//        '3' => return 3,
//        '4' => return 4,
//        '5' => return 5,
//        '6' => return 6,
//        '7' => return 7,
//        '8' => return 8,
//        '9' => return 9,
//        _   => return 0
//    }
    let option: Option<u32> = item.to_digit(BASE10);
    match option {
        Some(digit) => return digit,
        None        => return 0
    }
}

fn get_digits(number: String) -> [u32; NUMBER_SIZE] {
    let mut digits: [u32; NUMBER_SIZE] = [0; NUMBER_SIZE];
    let mut index: usize = 0;
    for item in number.chars() {
        digits[index] = char2digit(item);
        index += 1
    }
    digits
}

fn sum_cols(matrix: [[u32; NUMBER_SIZE]; SET_SIZE]) -> String {
    let mut result: String = "".to_string();
    let mut carryover: u32 = 0;
    let mut column: usize = NUMBER_SIZE;

    while column > 0 {

        let mut row: usize = 0;
        let mut sum: u32 = 0;
        column -= 1;

        let row_total: u32 = loop {
            sum += matrix[row][column];
            row += 1;
            if row >= SET_SIZE { break sum + carryover }
        };

        result = format!("{}{}", (row_total % 10).to_string(), result);
        carryover = row_total / 10 

    }

    if carryover > 0 { result = format!("{}{}", carryover.to_string(), result) }
    result[0..10].to_string()
}

pub fn compute() -> String {
    let mut digits = [[0u32; NUMBER_SIZE]; SET_SIZE];
    digits[0] = get_digits("37107287533902102798797998220837590246510135740250".to_string());
    digits[1] = get_digits("46376937677490009712648124896970078050417018260538".to_string());
    digits[2] = get_digits("74324986199524741059474233309513058123726617309629".to_string());
    digits[3] = get_digits("91942213363574161572522430563301811072406154908250".to_string());
    digits[4] = get_digits("23067588207539346171171980310421047513778063246676".to_string());
    digits[5] = get_digits("89261670696623633820136378418383684178734361726757".to_string());
    digits[6] = get_digits("28112879812849979408065481931592621691275889832738".to_string());
    digits[7] = get_digits("44274228917432520321923589422876796487670272189318".to_string());
    digits[8] = get_digits("47451445736001306439091167216856844588711603153276".to_string());
    digits[9] = get_digits("70386486105843025439939619828917593665686757934951".to_string());
    digits[10] = get_digits("62176457141856560629502157223196586755079324193331".to_string());
    digits[11] = get_digits("64906352462741904929101432445813822663347944758178".to_string());
    digits[12] = get_digits("92575867718337217661963751590579239728245598838407".to_string());
    digits[13] = get_digits("58203565325359399008402633568948830189458628227828".to_string());
    digits[14] = get_digits("80181199384826282014278194139940567587151170094390".to_string());
    digits[15] = get_digits("35398664372827112653829987240784473053190104293586".to_string());
    digits[16] = get_digits("86515506006295864861532075273371959191420517255829".to_string());
    digits[17] = get_digits("71693888707715466499115593487603532921714970056938".to_string());
    digits[18] = get_digits("54370070576826684624621495650076471787294438377604".to_string());
    digits[19] = get_digits("53282654108756828443191190634694037855217779295145".to_string());
    digits[20] = get_digits("36123272525000296071075082563815656710885258350721".to_string());
    digits[21] = get_digits("45876576172410976447339110607218265236877223636045".to_string());
    digits[22] = get_digits("17423706905851860660448207621209813287860733969412".to_string());
    digits[23] = get_digits("81142660418086830619328460811191061556940512689692".to_string());
    digits[24] = get_digits("51934325451728388641918047049293215058642563049483".to_string());
    digits[25] = get_digits("62467221648435076201727918039944693004732956340691".to_string());
    digits[26] = get_digits("15732444386908125794514089057706229429197107928209".to_string());
    digits[27] = get_digits("55037687525678773091862540744969844508330393682126".to_string());
    digits[28] = get_digits("18336384825330154686196124348767681297534375946515".to_string());
    digits[29] = get_digits("80386287592878490201521685554828717201219257766954".to_string());
    digits[30] = get_digits("78182833757993103614740356856449095527097864797581".to_string());
    digits[31] = get_digits("16726320100436897842553539920931837441497806860984".to_string());
    digits[32] = get_digits("48403098129077791799088218795327364475675590848030".to_string());
    digits[33] = get_digits("87086987551392711854517078544161852424320693150332".to_string());
    digits[34] = get_digits("59959406895756536782107074926966537676326235447210".to_string());
    digits[35] = get_digits("69793950679652694742597709739166693763042633987085".to_string());
    digits[36] = get_digits("41052684708299085211399427365734116182760315001271".to_string());
    digits[37] = get_digits("65378607361501080857009149939512557028198746004375".to_string());
    digits[38] = get_digits("35829035317434717326932123578154982629742552737307".to_string());
    digits[39] = get_digits("94953759765105305946966067683156574377167401875275".to_string());
    digits[40] = get_digits("88902802571733229619176668713819931811048770190271".to_string());
    digits[41] = get_digits("25267680276078003013678680992525463401061632866526".to_string());
    digits[42] = get_digits("36270218540497705585629946580636237993140746255962".to_string());
    digits[43] = get_digits("24074486908231174977792365466257246923322810917141".to_string());
    digits[44] = get_digits("91430288197103288597806669760892938638285025333403".to_string());
    digits[45] = get_digits("34413065578016127815921815005561868836468420090470".to_string());
    digits[46] = get_digits("23053081172816430487623791969842487255036638784583".to_string());
    digits[47] = get_digits("11487696932154902810424020138335124462181441773470".to_string());
    digits[48] = get_digits("63783299490636259666498587618221225225512486764533".to_string());
    digits[49] = get_digits("67720186971698544312419572409913959008952310058822".to_string());
    digits[50] = get_digits("95548255300263520781532296796249481641953868218774".to_string());
    digits[51] = get_digits("76085327132285723110424803456124867697064507995236".to_string());
    digits[52] = get_digits("37774242535411291684276865538926205024910326572967".to_string());
    digits[53] = get_digits("23701913275725675285653248258265463092207058596522".to_string());
    digits[54] = get_digits("29798860272258331913126375147341994889534765745501".to_string());
    digits[55] = get_digits("18495701454879288984856827726077713721403798879715".to_string());
    digits[56] = get_digits("38298203783031473527721580348144513491373226651381".to_string());
    digits[57] = get_digits("34829543829199918180278916522431027392251122869539".to_string());
    digits[58] = get_digits("40957953066405232632538044100059654939159879593635".to_string());
    digits[59] = get_digits("29746152185502371307642255121183693803580388584903".to_string());
    digits[60] = get_digits("41698116222072977186158236678424689157993532961922".to_string());
    digits[61] = get_digits("62467957194401269043877107275048102390895523597457".to_string());
    digits[62] = get_digits("23189706772547915061505504953922979530901129967519".to_string());
    digits[63] = get_digits("86188088225875314529584099251203829009407770775672".to_string());
    digits[64] = get_digits("11306739708304724483816533873502340845647058077308".to_string());
    digits[65] = get_digits("82959174767140363198008187129011875491310547126581".to_string());
    digits[66] = get_digits("97623331044818386269515456334926366572897563400500".to_string());
    digits[67] = get_digits("42846280183517070527831839425882145521227251250327".to_string());
    digits[68] = get_digits("55121603546981200581762165212827652751691296897789".to_string());
    digits[69] = get_digits("32238195734329339946437501907836945765883352399886".to_string());
    digits[70] = get_digits("75506164965184775180738168837861091527357929701337".to_string());
    digits[71] = get_digits("62177842752192623401942399639168044983993173312731".to_string());
    digits[72] = get_digits("32924185707147349566916674687634660915035914677504".to_string());
    digits[73] = get_digits("99518671430235219628894890102423325116913619626622".to_string());
    digits[74] = get_digits("73267460800591547471830798392868535206946944540724".to_string());
    digits[75] = get_digits("76841822524674417161514036427982273348055556214818".to_string());
    digits[76] = get_digits("97142617910342598647204516893989422179826088076852".to_string());
    digits[77] = get_digits("87783646182799346313767754307809363333018982642090".to_string());
    digits[78] = get_digits("10848802521674670883215120185883543223812876952786".to_string());
    digits[79] = get_digits("71329612474782464538636993009049310363619763878039".to_string());
    digits[80] = get_digits("62184073572399794223406235393808339651327408011116".to_string());
    digits[81] = get_digits("66627891981488087797941876876144230030984490851411".to_string());
    digits[82] = get_digits("60661826293682836764744779239180335110989069790714".to_string());
    digits[83] = get_digits("85786944089552990653640447425576083659976645795096".to_string());
    digits[84] = get_digits("66024396409905389607120198219976047599490197230297".to_string());
    digits[85] = get_digits("64913982680032973156037120041377903785566085089252".to_string());
    digits[86] = get_digits("16730939319872750275468906903707539413042652315011".to_string());
    digits[87] = get_digits("94809377245048795150954100921645863754710598436791".to_string());
    digits[88] = get_digits("78639167021187492431995700641917969777599028300699".to_string());
    digits[89] = get_digits("15368713711936614952811305876380278410754449733078".to_string());
    digits[90] = get_digits("40789923115535562561142322423255033685442488917353".to_string());
    digits[91] = get_digits("44889911501440648020369068063960672322193204149535".to_string());
    digits[92] = get_digits("41503128880339536053299340368006977710650566631954".to_string());
    digits[93] = get_digits("81234880673210146739058568557934581403627822703280".to_string());
    digits[94] = get_digits("82616570773948327592232845941706525094512325230608".to_string());
    digits[95] = get_digits("22918802058777319719839450180888072429661980811197".to_string());
    digits[96] = get_digits("77158542502016545090413245809786882778948721859617".to_string());
    digits[97] = get_digits("72107838435069186155435662884062257473692284509516".to_string());
    digits[98] = get_digits("20849603980134001723930671666823555245252804609722".to_string());
    digits[99] = get_digits("53503534226472524250874054075591789781264330331690".to_string());
    sum_cols(digits).to_string()
}
