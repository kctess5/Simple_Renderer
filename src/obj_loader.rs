#![allow(dead_code)]

use std::str::FromStr;
use std::io::{BufRead};
use std::iter::Filter;
use std::str::Split;

use glium::{self, Display};
use glium::vertex::VertexBufferAny;

type Words<'a> = Filter<Split<'a, fn(char) -> bool>, fn(&&str) -> bool>;

fn words<'a>(s: &'a str) -> Words<'a> {
	fn is_not_empty(s: &&str) -> bool { !s.is_empty() }
	let is_not_empty: fn(&&str) -> bool = is_not_empty; // coerce to fn pointer

	fn is_whitespace(c: char) -> bool { c.is_whitespace() }
	let is_whitespace: fn(char) -> bool = is_whitespace; // coerce to fn pointer!s.is_empty())

	s.split(is_whitespace).filter(is_not_empty)
}

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 3],
	normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);

impl Vertex {
	fn new(pos: [f32; 3], norm: [f32; 3]) -> Vertex {
		return Vertex{ position: pos, normal: norm }
	}
}

pub struct ObjBuffer {
	vertices: Vec<[f32; 3]>,
	normals: Vec<[f32; 3]>,
	objects: Vec<Vertex>,
}

impl ObjBuffer {
	fn new() -> ObjBuffer {
		ObjBuffer {
			vertices: Vec::new(),
			normals: Vec::new(),
			objects: Vec::new(),
		}
	}
	fn parse_vertex(&mut self, v0: Option<&str>, v1: Option<&str>, v2: Option<&str>) {
		let (v0, v1, v2) = match (v0, v1, v2) {
			(Some(v0), Some(v1), Some(v2)) => (v0, v1, v2),
			_ => { panic!("could not parse line {:?} {:?} {:?}", v0, v1, v2); }
		};
		let vertex = match (FromStr::from_str(v0), FromStr::from_str(v1), FromStr::from_str(v2)) {
			(Ok(v0), Ok(v1), Ok(v2)) => [v0, v1, v2],
			_ => { panic!("could not parse line {:?} {:?} {:?}", v0, v1, v2); }
		};
		self.vertices.push(vertex);
	}
	fn parse_normal(&mut self, n0: Option<&str>, n1: Option<&str>, n2: Option<&str>) {
		let (n0, n1, n2) = match (n0, n1, n2) {
			(Some(n0), Some(n1), Some(n2)) => (n0, n1, n2),
			_ => { panic!("could not parse line {:?} {:?} {:?}", n0, n1, n2); }
		};
		let normal = match (FromStr::from_str(n0), FromStr::from_str(n1), FromStr::from_str(n2)) {
			(Ok(n0), Ok(n1), Ok(n2)) => [n0, n1, n2],
			_ => { panic!("could not parse line {:?} {:?} {:?}", n0, n1, n2); }
		};
		self.normals.push(normal);
	}
	/// Add an object to the object buffer
	fn add_vertices(&mut self, vi: [usize; 3], ni: [usize; 3]) {
		for idx in 0..3 {
			let normal = self.normals[ ni[idx] - 1 ];
			let position = self.vertices[ vi[idx] - 1 ];

			self.objects.push(Vertex::new(position, normal));
		}
	}
	fn parse_face(&mut self, f0: Option<&str>, f1: Option<&str>, f2: Option<&str>) {
		let (f0, f1, f2) = match (f0, f1, f2) {
			(Some(f0), Some(f1), Some(f2)) => (f0, f1, f2),
			_ => { panic!("could not parse line {:?} {:?} {:?}", f0, f1, f2); }
		};

		let (mut g0, mut g1, mut g2) = (f0.split("/"), f1.split("/"), f2.split("/"));
		let (f0, f1, f2) = (g0.next(), g1.next(), g2.next());
		let (n0, n1, n2) = (g0.nth(1), g1.nth(1), g2.nth(1));

		let (n0, n1, n2, f0, f1, f2) = match (n0, n1, n2, f0, f1, f2) {
			(Some(n0), Some(n1), Some(n2), Some(f0), Some(f1), Some(f2)) => (n0, n1, n2, f0, f1, f2),
			_ => { panic!("could not parse face {:?} {:?} {:?} {:?} {:?} {:?}", n0, n1, n2, f0, f1, f2); }
		};

		let normal_index: [usize; 3] = match (FromStr::from_str(n0), FromStr::from_str(n1), FromStr::from_str(n2)) {
			(Ok(n0), Ok(n1), Ok(n2)) => [n0, n1, n2],
			_ => { panic!("could not parse normal {:?} {:?} {:?}", n0, n1, n2); }
		};

		let vertex_index: [usize; 3] = match (FromStr::from_str(f0), FromStr::from_str(f1), FromStr::from_str(f2)) {
			(Ok(f0), Ok(f1), Ok(f2)) => [f0, f1, f2],
			_ => { panic!("could not parse vertex {:?} {:?} {:?}", f0, f1, f2); }
		};
		
		self.add_vertices(vertex_index, normal_index);
	}
	pub fn load<B: BufRead>(input: &mut B) -> ObjBuffer {
		let mut dat = ObjBuffer::new();
		for line in input.lines() {
			let mut words = match line {
				Ok(ref line) => words(line),
				Err(err) => panic!("failed to readline {}", err)
			};
			match words.next() {
				Some("v") => { dat.parse_vertex(words.next(), words.next(), words.next());}
				Some("vn") => { dat.parse_normal(words.next(), words.next(), words.next());}
				Some("f") => { dat.parse_face(words.next(), words.next(), words.next());}
				_ => ()
			}
		}
		dat
	}
	pub fn load_std() -> ObjBuffer {
		use std::io::{self,BufRead};
		let mut dat = ObjBuffer::new();
		let stdin = io::stdin();

		for line in stdin.lock().lines() {
			let mut words = match line {
				Ok(ref line) => words(line),
				Err(err) => panic!("failed to readline {}", err)
			};
			match words.next() {
				Some("v") => { dat.parse_vertex(words.next(), words.next(), words.next());}
				Some("vn") => { dat.parse_normal(words.next(), words.next(), words.next());}
				Some("f") => { dat.parse_face(words.next(), words.next(), words.next());}
				_ => ()
			}
		}
		dat
	}
	pub fn to_vertex_buffer(&mut self, display: &Display) -> VertexBufferAny {
		glium::vertex::VertexBuffer::new(display, &self.objects.clone()).unwrap().into_vertex_buffer_any()
	}
	pub fn len(&self) -> usize {
		self.objects.len()
	}
}