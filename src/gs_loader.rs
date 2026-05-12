//  //  //  //  //  //  //  //
pub struct LoadedData3D {
    pub name: Box<str>,
    pub ijk: [usize; 3],
    pub gs_data: io3d::GSProperty,
}

//  //  //  //  //  //  //  //
impl LoadedData3D {
    pub fn from_filename(
        file_name: &str,
        ijk: &Option<Vec<usize>>,
    ) -> io3d::Result<LoadedData3D> {
        let (header, r) = io3d::GSHeader::from_filename(file_name)?;
        let ijk: [usize; 3] = if let Some(ijk) = ijk {
            println!("preset dimensions..");
            let ijk_len = ijk.len();
            if ijk_len != 3 {
                panic!("option --ijk must have 3 numers but got {}", ijk_len);
            }
            let ijk = ijk[0..3]
                .try_into()
                .expect("something goes wroing with --ijk");
            ijk
        } else {
            println!("autodimensions..");
            header.parse_ijk_dims()?.into()
        };
        let properties = io3d::GSProperty::from_gs_header(header, r)?;

        Ok(LoadedData3D {
            name: file_name.to_string().into_boxed_str(),
            ijk,
            gs_data: properties,
        })
    }
}
