use rand::prelude::*;
use serde_json;
use std::{collections::HashMap};
use std::cmp;
use serde::{Serialize, Deserialize};
//use serde_json::Result as SerdeResult;
///这是DND模拟器的库。六种属性的定义如下，来自第5版规则书。
///力量Strength，体能的量化	
///敏捷Dexterity，灵活度的量化	
///体质Constitution，耐受力的量化	
///智力Intelligence，记忆与思维能力的量化	
///感知Wisdom，直觉与感受能力的量化	
///魅力Charisma，个性气质的量化
/// 关于数值以及判定方法，请参考第5版规则书
/// 战斗步骤摘自规则书，如下：
/// 战斗步骤Combat	Step	by	Step	
///1. 判定突袭Determine	surprise。DM 判定战斗遭遇参与者中是否有人遭受突袭。	
///2. 决定位置 Establish	positions。DM 决定好所有角色和怪物的位置。
/// 即DM以冒险者的行进方向，及其在房间或其他地点的具体位置为基础，再确定其敌对者在哪（距离的远近和具体位置）。
/// 3. 骰先攻Roll	initiative。战斗遭遇的每位参与者投先攻骰，以决定战斗回合的顺序。
/// 4.执行回合Take	turns。战斗的每位参与者按照先攻序列进行其战斗回合。	
/// 5. 开始新一轮Begin	the	next	round。当所有战斗参与者完成其回合后，该轮结束。重复步骤4直至战斗结束。	
#[derive(Clone,Debug,Default,Serialize,Deserialize)]
///玩家的各种信息，注意由于rust的安全性，没有采用getter和setter方法，而是直接让字段可见
pub struct Player{
    ///玩家名称
    pub name:String,
    ///玩家6种属性值
    pub ability_scores:AbilityScores,
    pub coins:Coins,
    ///飞行速度\行走速度\护甲值\经验值\生命值在初始化时再设定
    pub walking_speed:i32,
    pub flying_speed:i32,
    pub armor:i32,
    pub exp:i32,
    pub hp:i32,
    ///通过玩家熟练项的名字（包括技能以及工具）查找对应的熟练项以及对应的熟练度加值
    /// 并且玩家职业本身带来的熟练项也直接写到对应哈希表中。哈希表加值为0含义是加上对应等级的加值
    /// 根据规则，一种技能/工具只能提供一个熟练度加值，并且我们不会添加新的属性，因此直接写死6个hashmap，下同
    /// ac即为AbilityCheck属性检定
    pub skills_for_ac_strength:HashMap<String,i32>,
    pub skills_for_ac_dexterity:HashMap<String,i32>,
    pub skills_for_ac_constitution:HashMap<String,i32>,
    pub skills_for_ac_intelligence:HashMap<String,i32>,
    pub skills_for_ac_wisdom:HashMap<String,i32>,
    pub skills_for_ac_charisma:HashMap<String,i32>,
    ///通过玩家熟练项的名字（包括技能以及工具）查找对应的熟练项以及对应的豁免加值
    /// st即为SavingThrow豁免检定
    pub skills_for_st_strength:HashMap<String,i32>,
    pub skills_for_st_dexterity:HashMap<String,i32>,
    pub skills_for_st_constitution:HashMap<String,i32>,
    pub skills_for_st_intelligence:HashMap<String,i32>,
    pub skills_for_st_wisdom:HashMap<String,i32>,
    pub skills_for_st_charisma:HashMap<String,i32>,
    // ///人物拥有的武器以及魔法
    pub weapons:HashMap<String,Weapon>,
    //pub tools:HashMap<String,Tool>,
}
#[derive(Copy,Clone,Debug,Default)]
///各种属性
pub enum Abilities{
    #[default] Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Copy,Clone,Debug,Serialize,Deserialize)]
///各种属性值
pub struct AbilityScores{
    pub strength:i32,
    pub dexterity:i32,
    pub constitution:i32,
    pub intelligence:i32,
    pub wisdom:i32,
    pub charisma:i32,
}
///默认属性值应该为15,14,13,12,10,8
impl Default for AbilityScores{
    fn default() -> Self {
        Self{
            strength:15,
            dexterity:14,
            constitution:13,
            intelligence:12,
            wisdom:10,
            charisma:8,
        }
    }
}

#[derive(Copy,Clone,Debug,Default)]
///调整值
pub struct Modifiers{
    pub strength:i32,
    pub dexterity:i32,
    pub constitution:i32,
    pub intelligence:i32,
    pub wisdom:i32,
    pub charisma:i32,
}

#[derive(Copy,Clone,Debug,Serialize,Deserialize)]
///默认的三种货币外加银金币和铂金币(1pp=10gp=20ep=100sp=1000cp)
pub struct Coins{
    pub gold:i32,
    pub silver:i32,
    pub copper:i32,
    pub ep:i32,
    pub pp:i32,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct HashedPlayers{
    pub hashed_players:HashMap<String,Player>,
}
#[derive(Copy,Clone,Debug,Serialize,Deserialize)]
pub enum CoinType{
    Gold,Silver,Copper,Ep,Pp
}
#[derive(Copy,Clone,Debug,Default,PartialEq)]
///每次检定的结果
pub enum DNDResult{
    #[default] Win,Tie,Lose
}
///默认货币数量值应该为10,50,100
impl Default for Coins{
    fn default() -> Self {
        Self { gold: 10, silver: 50, copper: 100,ep:0,pp:0 }
    }
}


#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Weapon{
    pub name:String,
    pub category:WeaponCategory,
    ///武器伤害(x,y)代表xdy，例如（1,6）代表1d6
    pub damage:(i32,i32), 
    pub damage_type:DamageType,
    pub price:(CoinType,i32),
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum WeaponCategory{
    SimpleMelee,
    SimpleRanged,
    MartialMelee,
    MartialRanged,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum DamageType{
    Blugeon,//钝击
    Slash,//劈砍
    Pierce,//穿刺
}
#[derive(Copy,Clone,Debug)]
///用于战斗时玩家的位置，并且战斗时玩家默认初始位置是50,50
pub struct Position{
    x:i32,y:i32
}
impl Default for Position{
    fn default() -> Self {
        Self { x: 50, y: 50 }
    }
}
///检定所需要的所有函数
pub trait DNDChecker{
    ///通过属性值产生调整值
    fn ability_scores_to_modifiers(&self)->Modifiers;
    ///获得全部可能的熟练加值
    fn proficiency_modifiers(&self)->Modifiers;
    ///获得全部可能的豁免加值
    fn saving_throw_modifiers(&self)->Modifiers;
    ///考虑d的大小的投掷 例如3D20就用这个骰子重复投3次
    fn dice(rng:&mut ThreadRng,upperbound:i32)->Result<i32,&'static str>;
    ///考虑d的大小，次数以及优劣势的投掷
    fn dice_complex(rng:&mut ThreadRng,upperbound:i32,count:i32,advantage:i32)->i32;
    ///属性检定，大于难度等级DifficultyClass即为成功，或者在需要时返回检定值
    /// 例如对于一次被动察觉检定，我们就需要返回检定值，并在此基础上进行进一步操作。
    fn ability_check(&self,checker:Abilities,dc:i32,count:i32,advantage:i32)->Result<DNDResult,&'static str>;
    fn ability_check_stat(&self,checker:Abilities,count:i32,advantage:i32)->Result<i32,&'static str>;
    ///豁免检定，大于难度等级DifficultyClass即为成功
    fn saving_throw(&self,checker:Abilities,dc:i32,count:i32,advantage:i32)->Result<DNDResult,&'static str>;
    fn saving_throw_stat(&self,checker:Abilities,count:i32,advantage:i32)->Result<i32,&'static str>;
}

///战斗所需要的5个步骤
///一场战斗中只能存在两个阵营。因此任何角色必须在开始或者中途加入某一方。
pub trait Combat where Self:DNDChecker+InformationGetter+InformationGetter{
    ///判定突袭，只在第一轮。对于所有选择隐匿的人，
    ///DM将躲藏方的隐匿结果即敏捷与另一方每个人的感知结果进行对抗。某个人结果为Lose说明被突袭成功。
    /// player_1,player_2分别代表两组战斗者（一场战斗只能有两组战斗者）。
    /// hide_1和hide_2分别代表每个人是(true)否(false)选择隐匿
    /// 返回的元组，第一个位置表示第一组战斗者，第二个位置表示第二组战斗者
    /// 每个表项中i32值为0表不存在对应给出名字的玩家，为1表示被突袭，为2表示没有被突袭
    fn determine_surprise(players_1:&HashMap<String,Player>,players_2:&HashMap<String,Player>,
    hide_1:&HashMap<String,bool>,hide_2:&HashMap<String,bool>)->
    (HashMap<String,i32>,HashMap<String,i32>);
    ///DM决定所有玩家以及怪物的位置
    fn establish_positions(players_1:&HashMap<String,Player>,players_2:&HashMap<String,Player>)->HashMap<String,Position>;
    ///所有参与者投掷先攻骰子确定顺序。返回的结果数值较小的代表顺序在前。
    fn roll_initiative(players_1:&HashMap<String,Player>,players_2:&HashMap<String,Player>)->HashMap<String,i32>;
    ///进行一轮，回合数即turn
    fn take_turns(players_1:&HashMap<String,Player>,players_2:&HashMap<String,Player>,turn:i32)->();
}
///查找或转换一些信息的函数
pub trait InformationGetter{
    ///经验值转化为等级，具体规则在代码中写出
    /// 注意在其他关联函数中exp小于0很可能导致panic
    fn exp_to_level(exp:i32)->Result<i32,&'static str>;
    ///将玩家等级或者怪物的挑战等级转化为熟练加值。Option返回None表示level不在范围
    fn level_to_proficiency_modifier(level:i32)->Option<i32>;
    ///将现有货币价值用(用户所需要类型货币数量，铜币数量）表示。如果转换为铜币，元组的第二位为0
    /// 1pp=10gp=20ep=100sp=1000cp
    /// ```
    /// use minidnd_eecs_havefun::{Player,Coins,CoinType,InformationGetter};
    /// assert_eq!(<Player as InformationGetter>::coins_to_coin(&Coins{gold:100,silver:0,copper:101,ep:1,pp:1},CoinType::Gold),Ok((111,51)));
    /// assert_eq!(<Player as InformationGetter>::coins_to_coin(&Coins{gold:1,silver:1,copper:1,ep:1,pp:1},CoinType::Copper),Ok((1161,0)));
    /// ```
    fn coins_to_coin(coins:&Coins,coin_type:CoinType)->Result<(i32,i32),&'static str>;
}
///用于读档、存档的函数
pub trait SaveLoad<T,F>{
    fn save_players(t:&mut F,file_name:&str)->Result<(),&'static str>;
    fn load_players(file_name:&str)->Result<HashMap<String,T>,&'static str>;
}
impl Player{
    pub fn new_by_default()->Player{
        Player { name: "Alice".to_string(),walking_speed:30,flying_speed:0,
        armor:8,exp:0,hp:100,..Default::default() }
    }
    pub fn new_by_stats(name:String,ability_scores:AbilityScores,coins:Coins,
        walking_speed:i32,flying_speed:i32,armor:i32,exp:i32,hp:i32)->Player{
        Player { name, ability_scores, coins,walking_speed,flying_speed,armor,exp,hp,..Default::default()}
    }
}

impl DNDChecker for Player{
    ///合法的难度范围是1到50，合法的d骰数量是1-10,合法的优劣势范围是-1到1（-1代表劣势，0代表没有优势或劣势，1代表优势）
    /// ```
    /// use minidnd_eecs_havefun::{Player,Abilities,DNDChecker};
    /// assert_eq!(Player::new_by_default().ability_check(Abilities::Strength,51,1,0),Err("dc is not in the range of 1 to 50\n"));
    /// assert_eq!(Player::new_by_default().ability_check(Abilities::Strength,15,11,0),Err("count is not in the range of 1 to 10\n"));
    /// assert_eq!(Player::new_by_default().ability_check(Abilities::Strength,15,1,-2),Err("advantage is not in the range of -1 to 1\n"));
    /// ```
    /// 默认情况下所有加值都被触发，但最多触发一个
    /// 本函数先调用不带DifficultyClass的函数获得结果再进行对比。
    fn ability_check(&self,checker:Abilities,dc:i32,count:i32,advantage:i32)->Result<DNDResult,&'static str> {
        if dc<1||dc>50 {Err("dc is not in the range of 1 to 50\n")}
        else if count<1||count>10 {Err("count is not in the range of 1 to 10\n")}
        else if advantage<(-1)||advantage>1{Err("advantage is not in the range of -1 to 1\n")}
        else{
            match self.ability_check_stat(checker,count, advantage){
                Ok(score)=>{
                    if score>=dc {Ok(DNDResult::Win)}
                    else if score==dc {Ok(DNDResult::Tie)}
                    else {Ok(DNDResult::Lose)}
                },
                Err(e)=>Err(e),
            }
        }
    }
    /// 合法的d骰数量是1-10,合法的优劣势范围是-1到1（-1代表劣势，0代表没有优势或劣势，1代表优势）
    /// ```
    /// use minidnd_eecs_havefun::{Player,Abilities,DNDChecker};
    /// assert_eq!(Player::new_by_default().ability_check_stat(Abilities::Strength,11,0),Err("count is not in the range of 1 to 10\n"));
    /// assert_eq!(Player::new_by_default().ability_check_stat(Abilities::Strength,1,-2),Err("advantage is not in the range of -1 to 1\n"));
    /// ```
    fn ability_check_stat(&self,checker:Abilities,count:i32,advantage:i32)->Result<i32,&'static str> {
        if count<1||count>10 {return Err("count is not in the range of 1 to 10\n")}
        else if advantage<(-1)||advantage>1{return Err("advantage is not in the range of -1 to 1\n")}
        else{
            let modifier=self.ability_scores_to_modifiers();
            let proficiency=self.proficiency_modifiers();
            let mut rng=rand::rng();
            let dice_result=Player::dice_complex(&mut rng,20,count,advantage);
            match checker{
                Abilities::Charisma=> {
                    let total_score=dice_result+modifier.charisma+proficiency.charisma;
                    Ok(total_score)
                },
                Abilities::Constitution=> {
                    let total_score=dice_result+modifier.constitution+proficiency.constitution;
                    Ok(total_score)
                },
                Abilities::Dexterity=> {
                    let total_score=dice_result+modifier.dexterity+proficiency.dexterity;
                    Ok(total_score)
                },
                Abilities::Intelligence=> {
                    let total_score=dice_result+modifier.intelligence+proficiency.intelligence;
                    Ok(total_score)
                },
                Abilities::Strength=> {
                    let total_score=dice_result+modifier.strength+proficiency.strength;
                    Ok(total_score)
                },
                Abilities::Wisdom=> {
                    let total_score=dice_result+modifier.wisdom+proficiency.wisdom;
                    Ok(total_score)
                },
            }
        }
    }
    fn ability_scores_to_modifiers(&self)->Modifiers{
        Modifiers{
            strength:(self.ability_scores.strength-10)/2,
            dexterity:(self.ability_scores.dexterity-10)/2,
            constitution:(self.ability_scores.constitution-10)/2,
            intelligence:(self.ability_scores.intelligence-10)/2,
            wisdom:(self.ability_scores.wisdom-10)/2,
            charisma:(self.ability_scores.charisma-10)/2,
        }
    }
    fn proficiency_modifiers(&self)->Modifiers {
        let mut max_strength=0;
        let mut max_dexterity=0;
        let mut max_constitution=0;
        let mut max_intelligence=0;
        let mut max_wisdom=0;
        let mut max_charisma=0;
        let level:i32=<Self as InformationGetter>::exp_to_level(self.exp).unwrap();
        let proficiency_modifier:i32=<Self as InformationGetter>::level_to_proficiency_modifier(level).unwrap();
        if !self.skills_for_ac_strength.is_empty(){
            max_strength=proficiency_modifier;
        }
        if !self.skills_for_ac_dexterity.is_empty(){
            max_dexterity=proficiency_modifier;
        }
        if !self.skills_for_ac_constitution.is_empty(){
            max_constitution=proficiency_modifier;
        }
        if !self.skills_for_ac_intelligence.is_empty(){
            max_intelligence=proficiency_modifier;
        }
        if !self.skills_for_ac_wisdom.is_empty(){
            max_wisdom=proficiency_modifier;
        }
        if !self.skills_for_ac_charisma.is_empty(){
            max_charisma=proficiency_modifier;
        }
        Modifiers { strength: max_strength, dexterity: max_dexterity,
             constitution: max_constitution, intelligence: max_intelligence,
             wisdom: max_wisdom, charisma: max_charisma, }
    }
    fn saving_throw_modifiers(&self)->Modifiers {
        let mut max_strength=0;
        let mut max_dexterity=0;
        let mut max_constitution=0;
        let mut max_intelligence=0;
        let mut max_wisdom=0;
        let mut max_charisma=0;
        let level:i32=<Self as InformationGetter>::exp_to_level(self.exp).unwrap();
        let proficiency_modifier:i32=<Self as InformationGetter>::level_to_proficiency_modifier(level).unwrap();
        if !self.skills_for_st_strength.is_empty(){
            max_strength=proficiency_modifier;
        }
        if !self.skills_for_st_dexterity.is_empty(){
            max_dexterity=proficiency_modifier;
        }
        if !self.skills_for_st_constitution.is_empty(){
            max_constitution=proficiency_modifier;
        }
        if !self.skills_for_st_intelligence.is_empty(){
            max_intelligence=proficiency_modifier;
        }
        if !self.skills_for_st_wisdom.is_empty(){
            max_wisdom=proficiency_modifier;
        }
        if !self.skills_for_st_charisma.is_empty(){
            max_charisma=proficiency_modifier;
        }
        Modifiers { strength: max_strength, dexterity: max_dexterity,
             constitution: max_constitution, intelligence: max_intelligence,
             wisdom: max_wisdom, charisma: max_charisma, }
    }
    ///单次投掷的上界范围应该是2-100
    /// ```
    /// use minidnd_eecs_havefun::{Player,DNDChecker,DNDResult};
    /// let mut rng=rand::rng();
    /// assert_eq!(Player::dice(&mut rng,101),Err("upperbound is not in the range of 2-100"));
    /// ```
    fn dice(rng:&mut ThreadRng,upperbound:i32)->Result<i32,&'static str> {
        if upperbound<2||upperbound>100{return Err("upperbound is not in the range of 2-100")}
        else {Ok(rng.random_range(1..upperbound+1))}
    }
    ///用户不应该自行调用这个函数
    ///注意用户给出的上界有误时这个函数将上界变为20.count有误时变为1.
    ///当要投的骰子数量是1，优势代表着再投掷一次，并且取两者较高值。劣势代表着再投掷一次，取两者较低值。
    ///只有骰子数量是1时优劣势才会生效
    /// ```
    /// use minidnd_eecs_havefun::{Player,DNDChecker,DNDResult};
    /// let mut rng=rand::rng();
    /// let dice_result=Player::dice_complex(&mut rng,20,1,1);
    /// ```
    fn dice_complex(rng:&mut ThreadRng,upperbound:i32,count:i32,advantage:i32)->i32 {
        if count<=1||count>10 {
            let dice_1=Player::dice(rng, upperbound).unwrap_or(Player::dice(rng, 20).unwrap());
            if advantage==0 {dice_1}
            else{
            let dice_2=Player::dice(rng, upperbound).unwrap_or(Player::dice(rng, 20).unwrap());
            if advantage==1 { cmp::max(dice_1,dice_2)}
            else {cmp::min(dice_1,dice_2)}
            }
        }
        else{
            let mut sum=0;
            let mut cnt=count;
            while cnt>=0{
                cnt-=1;
                let dice_1=Player::dice(rng, upperbound).unwrap_or(Player::dice(rng, 20).unwrap());
                sum+=dice_1;
            }
            sum
        }
    }
    ///只需要返回鉴定结果的豁免鉴定函数只是调用了带具体投掷结果的鉴定函数
    /// ```
    /// use minidnd_eecs_havefun::{Player,Abilities,DNDChecker,DNDResult};
    /// let my_player=Player::new_by_default();
    /// let dnd_result=
    /// my_player.saving_throw(Abilities::Strength,12,1,1)
    /// .unwrap_or_else(|e|{println!("Please check again,as {}",e);DNDResult::Tie});
    /// ```
    fn saving_throw(&self,checker:Abilities,dc:i32,count:i32,advantage:i32)->Result<DNDResult,&'static str> {
        if dc<1||dc>50 {Err("dc is not in the range of 1 to 50\n")}
        else if count<1||count>10 {Err("count is not in the range of 1 to 10\n")}
        else if advantage<(-1)||advantage>1{Err("advantage is not in the range of -1 to 1\n")}
        else{
            match self.saving_throw_stat(checker,count,advantage){
                Ok(score)=>{
                    if score>=dc {Ok(DNDResult::Win)}
                    else if score==dc {Ok(DNDResult::Tie)}
                    else {Ok(DNDResult::Lose)}
                },
                Err(e)=>Err(e),
            }
        }
    }
    /// 需要具体结果的鉴定函数。
    /// ```
    /// use minidnd_eecs_havefun::{Player,Abilities,DNDChecker,DNDResult};
    /// let mut my_player=Player::new_by_default();
    /// my_player.skills_for_st_strength.insert("skill_for_test".to_string(), 25);
    /// let dnd_stat=my_player.saving_throw_stat(Abilities::Strength,1,1)
    /// .unwrap_or_else(|e|{println!("Please check again,as {}",e);0});
    /// assert!(dnd_stat>=25,"dnd_stat={}",dnd_stat);
    /// ```
    fn saving_throw_stat(&self,checker:Abilities,count:i32,advantage:i32)->Result<i32,&'static str> {
        if count<1||count>10 {Err("count is not in the range of 1 to 10\n")}
        else if advantage<(-1)||advantage>1{Err("advantage is not in the range of -1 to 1\n")}
        else {
            let modifier=self.ability_scores_to_modifiers();
            let saving_throw=self.saving_throw_modifiers();
            let mut rng=rand::rng();
            let dice_result=Player::dice_complex(&mut rng,20,count,advantage);
            match checker{
                Abilities::Charisma => {
                    let total_score = dice_result + modifier.charisma + saving_throw.charisma;
                    Ok(total_score)
                },
                Abilities::Constitution => {
                    let total_score = dice_result + modifier.constitution + saving_throw.constitution;
                    Ok(total_score)
                },
                Abilities::Dexterity => {
                    let total_score = dice_result + modifier.dexterity + saving_throw.dexterity;
                    Ok(total_score)
                },
                Abilities::Intelligence => {
                    let total_score = dice_result + modifier.intelligence + saving_throw.intelligence;
                    Ok(total_score)
                },
                Abilities::Strength => {
                    let total_score = dice_result + modifier.strength + saving_throw.strength;
                    Ok(total_score)
                },
                Abilities::Wisdom => {
                    let total_score = dice_result + modifier.wisdom + saving_throw.wisdom;
                    Ok(total_score)
                },
            }
        }
    }
}
impl InformationGetter for Player{
    fn exp_to_level(exp:i32)->Result<i32,&'static str> {
        match exp{
            0..300=>Ok(1),
            300..900=>Ok(2),
            900..2700=>Ok(3),
            2700..6500=>Ok(4),
            6500..14000=>Ok(5),
            14000..23000=>Ok(6),
            23000..34000=>Ok(7),
            34000..48000=>Ok(8),
            48000..64000=>Ok(9),
            64000..85000=>Ok(10),
            85000..100000=>Ok(11),
            100000..120000=>Ok(12),
            120000..140000=>Ok(13),
            140000..165000=>Ok(14),
            165000..195000=>Ok(15),
            195000..225000=>Ok(16),
            225000..265000=>Ok(17),
            265000..305000=>Ok(18),
            305000..355000=>Ok(19),
            355000_i32..=i32::MAX=>Ok(20),
            _=>Err("your exp is not valid.")
        }
    }
    fn level_to_proficiency_modifier(level:i32)->Option<i32> {
        match level{
            1..=4=>Some(2),
            5..=8=>Some(3),
            9..=12=>Some(4),
            13..=16=>Some(5),
            17..=20=>Some(6),
            21..=24=>Some(7),
            25..=28=>Some(8),
            29..=30=>Some(9),
            _=>None,
        }
    }
    ///先把所有货币用铜币计数再用对应货币表示
    fn coins_to_coin(coins:&Coins,coin_type:CoinType)->Result<(i32,i32),&'static str> {
        let f=|c:&Coins|(c.gold*100+c.silver*10+c.copper+c.ep*50+c.pp*1000);
        let g=f(coins);
        if g<0 {return Err("Given Coins are wrong as the sum of them is negative")}
        let h=|n:i32,t:i32|((n/t,n-t*(n/t)));
        match coin_type{
            CoinType::Gold=>Ok(h(g,100)),
            CoinType::Silver=>Ok(h(g,10)),
            CoinType::Copper=>Ok(h(g,1)),
            CoinType::Ep=>Ok(h(g,50)),
            CoinType::Pp=>Ok(h(g,1000)),
        }
    }
}
impl SaveLoad<Player,HashedPlayers> for Player {
    fn load_players(file_name:&str)->Result<HashMap<String,Player>,&'static str> {
        use std::fs::File;
        use std::io::{BufReader};
        let file=File::open(file_name).map_err(|_|"Failed to open file")?;
        let reader = BufReader::new(file);
        let players:HashMap<String,Player>=serde_json::from_reader(reader).unwrap_or_default();
        Ok(players)
    }
    fn save_players(players:&mut HashedPlayers,file_name:&str)->Result<(),&'static str> {
        use std::fs::File;
        use std::io::{Write,BufWriter};
        let file=File::create(file_name).map_err(|_|"Failed to create file")?;
        let mut writer=BufWriter::new(file);
        match serde_json::to_string(&players.hashed_players){
                Ok(player_str)=>writer.write_all(player_str.as_bytes()).map_err(|_|"Failed to write player data\n")?,
                Err(_)=>return Err("Failed to serialize player\n")
        }
        writer.flush().map_err(|_|"Failed to flush writer\n")?;
        Ok(())
    }
}
impl Combat for Player {
    fn determine_surprise(players_1:&HashMap<String,Player>,players_2:&HashMap<String,Player>,
    hide_1:&HashMap<String,bool>,hide_2:&HashMap<String,bool>)->
    (HashMap<String,i32>,HashMap<String,i32>){
        //wisdom_1和wisdom_2用于存储players_1和players_2中所有player的感知固定值（基准10+能力调整值+熟练加值），注意不是检定值
        let mut wisdom_1:HashMap<String,i32>=HashMap::new();
        let mut wisdom_2:HashMap<String,i32>=HashMap::new();
        //考虑优势时这个值才有意义
        let tmp_modifier:i32=10;
        for (str,player) in players_1{
            wisdom_1.insert(str.clone(),player.ability_scores_to_modifiers().wisdom+player.proficiency_modifiers().wisdom+10);
        }
        for (str,player) in players_2{
            wisdom_2.insert(str.clone(),player.ability_scores_to_modifiers().wisdom+player.proficiency_modifiers().wisdom+10);
        }
        let mut ret_players_1:HashMap<String,i32>=HashMap::new();
        let mut ret_players_2:HashMap<String,i32>=HashMap::new();
        //对于每个阵营中需要执行隐匿的玩家，如果该玩家不在该阵营中，在对应阵营的返回哈希表中添加表象表示不存在;
        //如果存在，先计算该玩家的魅力检定值
        //对于每个敌方阵营中的玩家，依次用魅力检定值与对方阵营的感知固定值进行对抗，平局时算突袭未成功。
        for (str,_) in hide_1{
            match players_1.get(str){
                None=>{ret_players_1.insert(str.clone(), 0);}
                Some(s)=>{
                    let s_tmp_charisma=s.ability_check_stat(Abilities::Charisma, 1, 0).unwrap()
                +(s.ability_scores_to_modifiers().charisma)+(s.proficiency_modifiers().charisma);
                    for (str,_) in players_2{
                    let wisdom=*wisdom_1.get(str).unwrap();
                    if wisdom >=s_tmp_charisma{
                    ret_players_2.insert(str.clone(),2);
                    }
                    else {ret_players_2.insert(str.clone(),1);}
                }
            }
            }
        }
        for (str,_) in hide_2{
            match players_1.get(str){
                None=>{ret_players_2.insert(str.clone(), 0);}
                Some(s)=>{
                    let s_tmp_charisma=s.ability_check_stat(Abilities::Charisma, 1, 0).unwrap()
                +s.ability_scores_to_modifiers().charisma+s.proficiency_modifiers().charisma;
                    for (str,_) in players_1{
                    let wisdom=*wisdom_1.get(str).unwrap();
                    if wisdom >=s_tmp_charisma{
                    ret_players_1.insert(str.clone(),2);
                    }
                    else {ret_players_1.insert(str.clone(),1);}
                }
            }
            }
        }
        (ret_players_1,ret_players_2)
    }
    fn establish_positions(players_1:&HashMap<String,Player>,players_2:&HashMap<String,Player>)->HashMap<String,Position> {
        HashMap::<String,Position>::new()
    }
    fn roll_initiative(players_1:&HashMap<String,Player>,players_2:&HashMap<String,Player>)->HashMap<String,i32> {
        HashMap::<String,i32>::new()
    }
    fn take_turns(players_1:&HashMap<String,Player>,players_2:&HashMap<String,Player>,turn:i32)->() {
        
    }
}
