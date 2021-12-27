/// 东北天坐标系
#[derive(Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct Enu {
    /// 参考点向东，米
    pub e: f64,
    /// 参考点向北，米
    pub n: f64,
    /// 参考点向上，米
    pub u: f64,
}

/// WGS84 坐标系
#[derive(Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct WGS84 {
    /// 纬度，°
    pub latitude: f64,
    /// 经度，°
    pub longitude: f64,
    /// 海拔，米
    pub altitude: f64,
}

/// WGS84 与东北天互转的当地参考点，缓存了一些转换用到的中间变量
#[derive(Clone, Debug)]
pub struct LocalReference {
    origin: WGS84, // 东北天原点对应的 WGS84 参考点
    radius: f64,   // 参考点处的地球等效半径
    r_cos: f64,    // 参考点除的地球经线半径
}

impl From<WGS84> for LocalReference {
    fn from(origin: WGS84) -> Self {
        const A: f64 = 6378137.0;
        const B: f64 = A - A / 298.257223563;

        let (sin, cos) = origin.latitude.to_radians().sin_cos();
        let radius = (A * cos).hypot(B * sin) + origin.altitude;
        Self {
            origin,
            radius,
            r_cos: radius * cos,
        }
    }
}

impl LocalReference {
    #[inline]
    pub fn origin(&self) -> WGS84 {
        self.origin
    }

    pub fn wgs84_to_enu(&self, wgs84: WGS84) -> Enu {
        let d_latitude = wgs84.latitude - self.origin.latitude;
        let d_longitude = wgs84.longitude - self.origin.longitude;
        let d_altitude = wgs84.altitude - self.origin.altitude;
        Enu {
            e: self.r_cos * d_longitude.to_radians(),
            n: self.radius * d_latitude.to_radians(),
            u: d_altitude,
        }
    }

    pub fn enu_to_wgs84(&self, enu: Enu) -> WGS84 {
        let d_latitude = (enu.n / self.radius).to_degrees();
        let d_longitude = (enu.e / self.r_cos).to_degrees();
        let d_altitude = enu.u;
        WGS84 {
            latitude: self.origin.latitude + d_latitude,
            longitude: self.origin.longitude + d_longitude,
            altitude: self.origin.altitude + d_altitude,
        }
    }
}
