use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::sphere::Sphere;

pub struct HittableList
{
    pub objects: Vec<Sphere>
}

impl HittableList
{
    pub fn new() -> HittableList
    {
        HittableList { objects: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self)
    {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Sphere)
    {
        self.objects.push(object);
    }

    pub fn hit_closest_object(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>
    {
        let mut closest_so_far: f32 = t_max;
        let mut current_object: Option<HitRecord> = None;


        //loop over objects and check which one hit is the closest
        for object in &self.objects
        {
            let hit = object.hit(ray, t_min, closest_so_far);
            if hit.is_some()
            {
                let current_hit_distance = hit.as_ref().unwrap().distance;
                if current_hit_distance < closest_so_far
                {
                    closest_so_far = current_hit_distance;
                    current_object = hit;
                }
            }
        }

        return current_object;
    }
}
