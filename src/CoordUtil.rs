use eframe::egui::Pos2;

//serve per generare le coordinate relative rispetto 
//a quelle date dal painter
#[derive(Clone,Debug)]
pub struct CoordinateSystem{
    top:Pos2,
    bottom:Pos2
}



impl CoordinateSystem{

    pub fn from_clip_rect(clip_rect:egui::Rect) -> Self{
        Self { top: (clip_rect.min), bottom: (clip_rect.max) } 
    }
    pub fn max_x(&self) -> f32{
        self.bottom.x-self.top.x
    }
    pub fn max_y(&self) -> f32{
        self.bottom.y-self.top.y
    }

    pub fn to_cord(&self,x:f32,y:f32)->Pos2{
        Pos2{
            x:self.top.x+x,
            y:self.top.y+y
        }
    }
    pub fn in_area_option(&self,pos:Option<Pos2>)->bool{ 
        match pos {
        Some(pos) => self.in_area(pos),
        _ => false
        }
    }
    pub fn in_area(&self,pos:Pos2)->bool{ 
        (self.top.x < pos.x ) && (pos.x < self.bottom.x) && 
        (self.top.y < pos.y ) && (pos.y < self.bottom.y)
    }
}
