struct RemainedData{
    user_data: SnapShot
}

impl RemainedData{
   pub save_snapshot() -> Result<Ok(), Box<dyn Error::IO>>{
       // @TODO create let snapshot

       utilities::IO::write_file(snapshot)?;
   }
}
