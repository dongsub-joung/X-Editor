pub struct RemainedData{
    pub user_data: SnapShot
}

impl RemainedData{
   pub save_snapshot() -> Result<Ok(), Box<dyn Error::IO>>{
       // @TODO create let snapshot
       utilities::IO::write_file(&snapshot)?;

       RemainedData::user_data= snapshot;
       Ok(_)
   }
}
