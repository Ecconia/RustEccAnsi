use ecc_ansi_lib::arg_wrapper;
use ecc_ansi_lib_proc::ansi;

fn main() {
	println!(ansi!("«r»R«g»G«b»B «y»Y«p»P«t»T «w»W«s»S«» «80,255,80»RGB"));
	println!(ansi!("«dr»R«dg»G«db»B «dy»Y«dp»P«dt»T «w»W«s»S«» «4FC5F8»HEX"));
	println!(ansi!("«lr»R«lg»G«lb»B «ly»Y«lp»P«lt»T «w»W«s»S«»"));
	println!(arg_wrapper!(
		"asdf{}{}asdf{}asdf", "s", "123,123,255"
	), "test1", "test2", "test3");
	println!(arg_wrapper!(
		"{}asdf{}{}asdf{}asdf{}", "s", "123,255,123"
	), "testS", "test1", "test2", "test3", "testE");
}
