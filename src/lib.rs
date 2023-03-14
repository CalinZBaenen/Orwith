/// Represents one, another, or two values. (Addtionally, as an edgecase, the `enum` contains a `Neither` value.)
/// 
/// # Example
/// ```rust
/// use orwith::Orwith;
/// 
/// 
/// enum Component {
/// 	Audio {/* ... */},
/// 	Video {/* ... */}
/// }
/// 
/// impl Component {
/// 	fn audio() -> Self { Self::Audio {} }
/// 	fn video() -> Self { Self::Video {} }
/// }
/// 
/// 
/// let audio_components = Some(Component::audio(/* ... */));
/// let video_components = Some(Component::video(/* ... */));
/// 
/// let alternatives = Orwith::orwith(audio_components, video_components);
/// ```
pub enum Orwith<T> {
	/// Neither option was chosen or both options were refused.
	Neither,
	/// Both options were preferred.
	With(T, T),
	/// Only one option was preferred or the option not chosen was refused.
	Or(T)
}

impl<T> Orwith<T> {
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