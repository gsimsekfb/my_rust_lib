
// -----------------------------------------------------------------
// Implementing an Object-Oriented Design Pattern with trait object vs 
// encoding state into the type system
// -----------------------------------------------------------------


// todo




// 1. Blog Post impl. with trait object (object oriented design pattern)
//    - Behavior: in State trait
//    - Data    : in Post struct
//
// 
// Trait State
//   - request_review returns new State
// Struct Post with State trait

trait State {    
  // take ownership of Box<Self>, invalidating the old state so the state 
  // value can transform into a new state
  fn request_review(self: Box<Self>) -> Box<dyn State>;
      // !! this method is valid only when called on a Box holding the type
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, post: &'a Post) -> &'a str { "" }
      // Default implementation for the content method.
      // This means this fn doesn’t need to be implemented by client structs.
}

pub struct Post {
  state: Option<Box<dyn State>>, // trait object of Box<dyn State>
      // Why Option? So that we can call the take() method to take the 
      // Some value out of the state field and  leave a None in its place.
      // This lets us move the state value out of Post rather than borrowing it.
  content: String,
}

impl Post {
  pub fn new() -> Post {
      Post {
          state: Some(Box::new(Draft {})),
          content: String::new(),
      }
  }

  pub fn add_text(&mut self, text: &str) {
      self.content.push_str(text);
  }

  pub fn content(&self) -> &str {
      self.state.as_ref().unwrap().content(self)
  }

  pub fn request_review(&mut self) {
      if let Some(state) = self.state.take() {
          self.state = Some(state.request_review())
      }
  }

  pub fn approve(&mut self) {
      if let Some(s) = self.state.take() {
          self.state = Some(s.approve())
      }
  }
}


// State structs 

struct Draft {}
impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
      Box::new(PendingReview {})
  }
  fn approve(self: Box<Self>) -> Box<dyn State> { self }
}

struct PendingReview {}
impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> { self }
  fn approve(self: Box<Self>) -> Box<dyn State> {
      Box::new(Published {})
  }    
}

struct Published {}
impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> { self }
  fn approve(self: Box<Self>) -> Box<dyn State> { self }
  fn content<'a>(&self, post: &'a Post) -> &'a str { 
      &post.content
  }
}

#[test] fn ex1() {
  let mut post = Post::new();
  post.add_text("abc");
  assert_eq!("", post.content());
  post.request_review();
  assert_eq!("", post.content());
  post.approve();
  assert_eq!("abc", post.content());
}




// =======================



// 2. Encoding state into the type system 
//    (compile time check for state transitions)

pub struct Post_ { content: String }
impl Post_ {
  pub fn new_draft_post() -> DraftPost {
      DraftPost { content: "".to_string() }
  }
  pub fn content(&self) -> &str { &self.content }
}

pub struct DraftPost { content: String }
impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
      self.content.push_str(text);
  }
  pub fn request_review(self) -> PendingReviewPost {
      PendingReviewPost { content: self.content }
  }    
}

pub struct PendingReviewPost { content: String }
impl PendingReviewPost {
  pub fn approve(self) -> Post_ {
      Post_ { content: self.content }
  }
}

#[test] fn ex2() {
  let mut draft_post = Post_::new_draft_post();
  draft_post.add_text("abc");
  let pending_review_post = draft_post.request_review();
  let approved_post = pending_review_post.approve();
  assert_eq!("abc", approved_post.content());
}