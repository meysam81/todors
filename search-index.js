var searchIndex = JSON.parse('{\
"todors":{"doc":"","t":[0,0,0,5,0,0,3,4,3,13,3,13,3,13,3,13,13,13,13,4,13,3,3,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,11,11,11,11,11,11,11,11,11,12,12,12,12,11,11,11,11,11,11,11,11,11,12,5,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,5,5,5,3,11,11,11,11,12,11,11,11,11,12,11,11,11,11,12,11,11,11,11,3,11,11,12,5,5,11,11,11,11,12,11,11,11,11],"n":["cli","db","logging","main","models","settings","Cli","Commands","Completion","Completion","Create","Create","Delete","Delete","Get","Get","Grpc","Http","List","Serve","Serve","ServerAddr","Update","Update","augment_args","augment_args","augment_args","augment_args","augment_args","augment_args","augment_args","augment_args_for_update","augment_args_for_update","augment_args_for_update","augment_args_for_update","augment_args_for_update","augment_args_for_update","augment_args_for_update","augment_subcommands","augment_subcommands","augment_subcommands_for_update","augment_subcommands_for_update","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","command","command","command_for_update","done","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches_mut","from_arg_matches_mut","from_arg_matches_mut","from_arg_matches_mut","from_arg_matches_mut","from_arg_matches_mut","from_arg_matches_mut","from_arg_matches_mut","from_arg_matches_mut","generate_completions","group_id","group_id","group_id","group_id","group_id","group_id","group_id","has_subcommand","has_subcommand","host","id","id","id","into","into","into","into","into","into","into","into","into","port","print_completions","shell","title","title","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","undone","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches_mut","update_from_arg_matches_mut","update_from_arg_matches_mut","update_from_arg_matches_mut","update_from_arg_matches_mut","update_from_arg_matches_mut","update_from_arg_matches_mut","update_from_arg_matches_mut","update_from_arg_matches_mut","connect","migrate","get_log_level","init","Todo","borrow","borrow_mut","delete","deserialize","done","fmt","from","from_row","get","id","into","list","new","save","title","try_from","try_into","type_id","update","Settings","borrow","borrow_mut","db_url","default_dburl","default_loglevel","deserialize","fmt","from","into","log_level","new","try_from","try_into","type_id"],"q":["todors","","","","","","todors::cli","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","todors::db","","todors::logging","","todors::models","","","","","","","","","","","","","","","","","","","","todors::settings","","","","","","","","","","","","","",""],"d":["","","","","","","","","","Generate shell completion","","Create a new TODO with a title","","Delete a TODO by ID","","Get a TODO by ID","Serve gRPC over HTTP server","Serve REST over HTTP server","List all TODOs","","Serve either the gRPC or REST over HTTP server","","","Update a TODO by ID","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The ID of the TODO to delete","The ID of the TODO to update","The ID of the TODO to get","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","Generate completion scripts for your shell","The title of the TODO","The new title of the TODO","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","",""],"i":[0,0,0,0,0,0,0,0,0,8,0,8,0,8,0,8,9,9,8,0,8,0,0,8,5,10,11,12,13,14,15,5,10,11,12,13,14,15,8,9,8,9,5,8,9,10,11,12,13,14,15,5,8,9,10,11,12,13,14,15,5,5,5,13,5,8,9,10,11,12,13,14,15,5,8,9,10,11,12,13,14,15,5,8,9,10,11,12,13,14,15,5,8,9,10,11,12,13,14,15,0,5,10,11,12,13,14,15,8,9,10,12,13,14,5,8,9,10,11,12,13,14,15,10,0,15,11,13,5,8,9,10,11,12,13,14,15,5,8,9,10,11,12,13,14,15,5,8,9,10,11,12,13,14,15,13,5,8,9,10,11,12,13,14,15,5,8,9,10,11,12,13,14,15,0,0,0,0,0,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,0,35,35,35,0,0,35,35,35,35,35,35,35,35,35],"f":[0,0,0,[[],[[3,[[2,[1]]]]]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[4,4],[4,4],[4,4],[4,4],[4,4],[4,4],[4,4],[4,4],[4,4],[4,4],[4,4],[4,4],[4,4],[4,4],[4,4],[4,4],[4,4],[4,4],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],4],0,[[],4],0,[[5,6],7],[[8,6],7],[[9,6],7],[[10,6],7],[[11,6],7],[[12,6],7],[[13,6],7],[[14,6],7],[[15,6],7],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[16,[[3,[5,17]]]],[16,[[3,[8,17]]]],[16,[[3,[9,17]]]],[16,[[3,[10,17]]]],[16,[[3,[11,17]]]],[16,[[3,[12,17]]]],[16,[[3,[13,17]]]],[16,[[3,[14,17]]]],[16,[[3,[15,17]]]],[16,[[3,[5,17]]]],[16,[[3,[8,17]]]],[16,[[3,[9,17]]]],[16,[[3,[10,17]]]],[16,[[3,[11,17]]]],[16,[[3,[12,17]]]],[16,[[3,[13,17]]]],[16,[[3,[14,17]]]],[16,[[3,[15,17]]]],[[18,4]],[[],[[20,[19]]]],[[],[[20,[19]]]],[[],[[20,[19]]]],[[],[[20,[19]]]],[[],[[20,[19]]]],[[],[[20,[19]]]],[[],[[20,[19]]]],[21,22],[21,22],0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,[23],0,0,0,[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],3],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],0,[[5,16],[[3,[17]]]],[[8,16],[[3,[17]]]],[[9,16],[[3,[17]]]],[[10,16],[[3,[17]]]],[[11,16],[[3,[17]]]],[[12,16],[[3,[17]]]],[[13,16],[[3,[17]]]],[[14,16],[[3,[17]]]],[[15,16],[[3,[17]]]],[[5,16],[[3,[17]]]],[[8,16],[[3,[17]]]],[[9,16],[[3,[17]]]],[[10,16],[[3,[17]]]],[[11,16],[[3,[17]]]],[[12,16],[[3,[17]]]],[[13,16],[[3,[17]]]],[[14,16],[[3,[17]]]],[[15,16],[[3,[17]]]],[[21,[20,[25]]],[[3,[26,27]]]],[26,[[3,[27]]]],[21,28],[21,29],0,[[]],[[]],[[25,26],[[3,[30,27]]]],[[],[[3,[31]]]],0,[[31,6],7],[[]],[[],[[32,[31]]]],[[25,26],[[3,[31,27]]]],0,[[]],[26,[[3,[[33,[31]],27]]]],[34,31],[[31,26],[[3,[27]]]],0,[[],3],[[],3],[[],24],[[25,[20,[34]],[20,[22]],26],[[3,[[2,[1]]]]]],0,[[]],[[]],0,[[],34],[[],34],[[],[[3,[35]]]],[[35,6],7],[[]],[[]],0,[[],[[3,[35,36]]]],[[],3],[[],3],[[],24]],"p":[[8,"Error"],[3,"Box"],[4,"Result"],[3,"Command"],[3,"Cli"],[3,"Formatter"],[6,"Result"],[4,"Commands"],[4,"Serve"],[3,"ServerAddr"],[3,"Create"],[3,"Delete"],[3,"Update"],[3,"Get"],[3,"Completion"],[3,"ArgMatches"],[6,"Error"],[8,"Generator"],[3,"Id"],[4,"Option"],[15,"str"],[15,"bool"],[4,"Shell"],[3,"TypeId"],[15,"u32"],[6,"SqlitePool"],[4,"Error"],[4,"Level"],[3,"Logger"],[3,"SqliteQueryResult"],[3,"Todo"],[6,"Result"],[3,"Vec"],[3,"String"],[3,"Settings"],[4,"ConfigError"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};