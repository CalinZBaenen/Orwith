/// Represents one, another, or two values. (Addtionally, as an edgecase, the `enum` contains a `Neither` value.)
/// 
/// # Example
/// ```rust
/// use orwith::Orwith;
/// 
/// let x:Option<&str> = Some("Hello!");
/// let y:Option<&str> = Some("Goodbye!");
/// 
/// let v = Orwith::orwith(x, y); // Orwith::With(...)
/// ```
#[derive(Debug)]
pub enum Orwith<T> {
	/// Neither option was chosen or both options were refused.
	Neither,
	/// Both options were preferred.
	With(T, T),
	/// Only one option was preferred or the option not chosen was refused.
	Or(T)
}

impl<T> Orwith<T> {
	/// Takes two options and decides which variant to choose based on the presence of values.
	pub fn orwith(x:Option<T>, y:Option<T>) -> Self {
		match (x, y) {
			(Some(vx), Some(vy))        => Self::With(vx, vy),
			(Some(v), _) | (_, Some(v)) => Self::Or(v),
			_                           => Self::Neither
		}
	}
	
	
	
	/// Returns a mutable reference to the first value.
	pub fn first_mut(&mut self) -> Option<&mut T> {
		match self {
			Self::Neither    => None,
			Self::With(v, _) => Some(v),
			Self::Or(v)      => Some(v)
		}
	}
	
	/// Returns a mutable reference to the last value.
	pub fn last_mut(&mut self) -> Option<&mut T> {
		match self {
			Self::With(_, v) => Some(v),
			_ => self.first_mut()
		}
	}
	
	/// Returns the first item and consumes this `Orwith` value.
	pub fn to_first(self) -> Option<T> { self.to_pair().0 }
	
	/// Returns the last item and consumes this `Orwith` value.
	pub fn to_last(self) -> Option<T> { self.to_pair().1 }
	
	/// Returns the first and last items and consumes this `Orwith` value.
	pub fn to_pair(self) -> (Option<T>, Option<T>) {
		let mut p = (None, None);
		match self {
			Self::With(x, y) => { (p.0, p.1) = (Some(x), Some(y)); }
			Self::Or(v) => { (p.0, p.1) = (Some(v), None); }
			_ => {}
		}
		
		p
	}
	
	/// Returns an immutable reference to the first value.
	pub fn first(&self) -> Option<&T> {
		match self {
			Self::Neither    => None,
			Self::With(v, _) => Some(&v),
			Self::Or(v)      => Some(&v)
		}
	}
	
	/// Returns an immutable reference to the last value.
	pub fn last(&self) -> Option<&T> {
		match self {
			Self::With(_, v) => Some(v),
			_ => self.first()
		}
	}
}

impl<T> Default for Orwith<T> {
	fn default() -> Self { Self::Neither }
}

impl<T> From<(Option<T>, Option<T>)> for Orwith<T> {
	fn from(v:(Option<T>, Option<T>)) -> Self { Self::orwith(v.0, v.1) }
}

impl<T> Into<(Option<T>, Option<T>)> for Orwith<T> {
	fn into(self) -> (Option<T>, Option<T>) { self.to_pair() }
}