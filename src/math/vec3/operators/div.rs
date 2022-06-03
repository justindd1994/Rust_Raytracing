use std::ops;
use crate::vec3::Vec3;

//#region Operator /
impl ops::Div<Vec3> for Vec3
{
    type Output = Vec3;
    fn div(self, b: Vec3) -> Self::Output
    {
        let a = self;
        Vec3 { 
            x: a.x / b.x,
            y: a.y / b.y,
            z: a.z / b.z
        }
    }
}

impl ops::Div<&Vec3> for Vec3
{
    type Output = Vec3;
    fn div(self, b: &Vec3) -> Self::Output
    {
        let a = self;
        Vec3 { 
            x: a.x / b.x,
            y: a.y / b.y,
            z: a.z / b.z
        }
    }
}

impl ops::Div<Vec3> for &Vec3
{
    type Output = Vec3;
    fn div(self, b: Vec3) -> Self::Output
    {
        let a = self;
        Vec3 { 
            x: a.x / b.x,
            y: a.y / b.y,
            z: a.z / b.z
        }
    }
}

impl ops::Div<&Vec3> for &Vec3
{
    type Output = Vec3;
    fn div(self, b: &Vec3) -> Self::Output
    {
        let a = self;
        Vec3 { 
            x: a.x / b.x,
            y: a.y / b.y,
            z: a.z / b.z
        }
    }
}

impl ops::Div<f32> for Vec3
{
    type Output = Vec3;
    fn div(self, b: f32) -> Self::Output
    {
        let a = self;
        Vec3 { 
            x: a.x / b,
            y: a.y / b,
            z: a.z / b
        }
    }
}

impl ops::Div<f32> for &Vec3
{
    type Output = Vec3;
    fn div(self, b: f32) -> Self::Output
    {
        let a = self;
        Vec3 { 
            x: a.x / b,
            y: a.y / b,
            z: a.z / b
        }
    }
}

impl ops::Div<&f32> for Vec3
{
    type Output = Vec3;
    fn div(self, b: &f32) -> Self::Output
    {
        let a = self;
        Vec3 { 
            x: a.x / b,
            y: a.y / b,
            z: a.z / b
        }
    }
}

impl ops::Div<&f32> for &Vec3
{
    type Output = Vec3;
    fn div(self, b: &f32) -> Self::Output
    {
        let a = self;
        Vec3 { 
            x: a.x / b,
            y: a.y / b,
            z: a.z / b
        }
    }
}
//#endregion
