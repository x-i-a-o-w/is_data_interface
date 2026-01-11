use super::*;

// having RecoverInterfaceRead is dumb why not just fuse the read and write variant into one?
// this could cause problems for scalability of interfaces which aren't like the filesystem(for structures of non public access?)

// InterfaceRead has the ability to fix the InterfaceRead if read returns Err(e)
// recover may fail if InterfaceRead cannot be made with write
// this function should only fail because of stuff not existing or not having perms or something that he user cannot modify thru program himself
// pub trait RecoverInterfaceRead<'input>: InterfaceRead<'input> {
//     fn recover(input: Self::Input) -> std::result::Result<Self::Output, Self::OutputError>;
// }
// 		fn read_recover(input: Self::Input)->std::result::Result<Self::Output, Self::OutputError>{
// match Self::read(input){
// 	Ok(o)=>{Ok(o)},
// 	Err(e)=>{
// 		match Self::recover(input){
// 			Ok(o)=>{o},
// 			Err(e)=>{},
// 		}
// 	},
// }
//   }

// pub trait RecoverInterfaceWrite<'input>: InterfaceWrite<'input> {
//     fn recover(input: Self::Input) -> std::result::Result<Self::Output, Self::OutputError>;
//     fn write_recover(input: Self::Input)->std::result::Result<Self::Output, Self::OutputError>{
//     	Self::read(input)
//     }
// }

// pub trait RecoverInterfaceReadWrite<'input>: InterfaceWrite<'input>+InterfaceRead<'input> {}
