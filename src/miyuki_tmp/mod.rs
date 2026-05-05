struct RemainedData{
    user_data: SnapShort
}

impl RemainedData{
   pub new() -> Self{
       let user_data= SnapShort::new();
        self { user_data }
   }
}
