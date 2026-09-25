pub const ESIK_YUZ: f32 = 0.65;
pub const ESIK_PRESENCE:f32 = 0.5;
pub const ESIK_LIVENESS:f32 = 0.6;
pub const MIN_TRACK_SURESI:f32 = 3.0;
#[derive(Debug,Clone)]
pub struct VisionScores {
    pub presence:f32,
    pub yuz:f32,
    pub liveness:f32,
    pub track_sure:f32,
    pub whitelist:bool,
    pub birlikte_mi:bool,
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
    pub enum Decision{
        OtoAc,EslikliAc,Sor,Reddet,
    }
impl Decision{
    pub fn as_str(&self)->&'static str{
        match self{
            Decision::OtoAc => "oto_ac",
            Decision::EslikliAc=> "eslikli_ac",
            Decision::Sor => "soruldu",
            Decision::Reddet=>"red",
        }
    }
}
pub fn validate(s:&VisionScores) ->Result<(),&'static str> {
    for v in [s.yuz,s.presence,s.liveness]   
    {
        if!(0.0..=1.0).contains(&v) || !v.is_finite(){
            return Err("skorlar [0,1] araliginda ve sonlu olmalidir");
        }
    }
    if !s.track_sure.is_finite() || s.track_sure < 0.0 {
        return Err("track_sure gecerli bir sure olmalidir");
    }
    Ok(())
}
pub fn decide(s:&VisionScores) -> (Decision,&'static str){
    if s.presence < ESIK_PRESENCE {
        return (Decision::Reddet,"BEDEN ALGILANAMADI");
    }
    if s.liveness < ESIK_LIVENESS {
        return(Decision::Reddet,"CANLILIK ALGILANAMADI");
    }
    if s.track_sure < MIN_TRACK_SURESI {
        return (Decision::Reddet,"algılanacak kadar durmadı");
    }
    if s.yuz < ESIK_YUZ {
        if s.birlikte_mi{
            return(Decision::EslikliAc,"COKLU GIRIS");
        }
    return (Decision::Sor,"YUZ TANINAMADI.BILDIRIM GONDER");
    }
    if s.whitelist {
        return(Decision::OtoAc,"BILINEN KISI GIRIS");
    }
    (Decision::Sor,"kapıda biri var.tanımıyoruz.")
}
#[cfg(test)]
mod tests{
    use super::*;
fn olay(yuz:f32,presence:f32,liveness:f32,sure:f32,wl:bool,birlikte:bool) 
-> VisionScores{
    VisionScores{
        presence,yuz,liveness,
        track_sure: sure,
        whitelist: wl,
        birlikte_mi:birlikte
    }
}

#[test]
fn anne_oto_ac (){
    let (karar,_) = decide(&olay(0.93,0.9,0.85,4.2,true,false));
    assert_eq!(karar,Decision::OtoAc);
    }

#[test]
fn elde_foto_red(){
    let (karar,_) =decide(&olay(0.91,0.2,0.1,4.0,false,false));
    assert_eq!(karar,Decision::Reddet);
}
#[test]
fn bozuk_skor_red(){
  let s = olay(9.9,0.9,0.8,4.0,true,false); 
  assert!(validate(&s).is_err());  
}
}