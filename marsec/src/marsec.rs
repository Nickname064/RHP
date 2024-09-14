
/*
const DIRECTIVE_NAMES: &[&str] = &[
    "define", //Define a custom element
             //"import", //Import another page's custom elements
             //"insert" //Insert the contents of another page here
];

enum MarsecError<'a>{
    ValuelessQuery,
    InvalidQuery(&'a str)
}

enum MarsecElement<'a>{
    HTML(HTMLEnum<'a>),
    Marsec(MarsecToken<'a>),
}

struct CustomTag<'a> {
    pub tagname: String,
    pub contents: Vec<MarsecElement<'a>>
}

impl CustomTag<'_>{

    /// Given a list of the defined custom tags,
    /// computes the dependencies of this tag.
    ///
    /// Circular tag invocations are forbidden, so when they happen,
    /// the tag isnt processed a second time to avoid infinite loops
    ///
    /// This mean that a tag cannot be included in its own dependencies
    fn get_deps(&self, custom_tags : &str) -> Vec<String> {

        let mut q : VecDeque<&MarsecElement> = VecDeque::new();
        let _ = self.contents.iter().for_each(|x| q.push_back(x));

        let mut results = vec![];

        loop{
            match q.pop_front(){
                None => { break; }
                Some(MarsecElement::Marsec(DEMUX { contents, ..})) => {
                    contents.iter().for_each(|x| q.push_back(x));
                }
                Some(MarsecElement::HTML(Node(html_ref))) => {
                    //Treat current node
                    let borrow = html_ref.borrow();
                    if borrow.name() != self.tagname && custom_tags.contains(borrow.name()){
                        results.push(String::from(borrow.name()));
                    }

                    //Treat all of its children
                    for elem in borrow.rec_html_children() {
                        let vborrow = elem.borrow();
                        if vborrow.name() != self.tagname && custom_tags.contains(borrow.name()){
                            results.push(String::from(vborrow.name()));
                        }
                    }
                }
                _ => { /* TEXT NODES, COMMENTS, AND CHILDLESS MARSEC NODES DONT AFFECT DEPENDENCIES */ }
            }
        }

        results.dedup();
        results
    }

}

enum Directives<'a> {
    ///DEFINE a new custom tag
    DEFINE(CustomTag<'a>),
}

impl Directives<'_>{
    pub fn from_html<'a>(html : HTMLNode<'a>) -> Directives<'a>{

        match html.name.to_lowercase().as_str(){
            "define-tag" => {
                if let Some( (tagname, None) ) = html.attributes.iter().nth(0){
                    Directives::DEFINE(CustomTag {
                        tagname: String::from(*tagname),
                        contents: html.children.into_iter().map(|x| {
                            MarsecElement::HTML(x)
                        }).collect()
                    })

                } else{
                    panic!("Unnamed define tag");
                }
            }
            _ => { panic!("Unknown directive") }
        }
    }
}

enum MarsecToken<'a> {
    DEMUX{ query : HCombinedQuery<'a>, contents : Vec<MarsecElement<'a>> },
    CHILDREN{ query : HCombinedQuery<'a> }
}

impl<'a> MarsecToken<'a>{
    fn apply_to(&self, tag_invocation : &HTMLNode<'a>, children : Vec<HTMLEnum<'a>>) -> Vec<HTMLEnum<'a>>{
        match self{
            CHILDREN{ query } => {

                //Simply paste the children that match the query

                children.iter()
                    .filter(|x| {
                        if let Node(noderef) = x {
                            query.matches(noderef.borrow().deref())
                        } else{ true }
                    }).map(|x| x.clone()).collect()
            }
            DEMUX{ query, contents} => {

                //Filter the children by the query, then recursively apply the child tags to them

                let filtered_children : Vec<HTMLEnum> = children.iter()
                    .filter(|x| {
                        if let Node(noderef) = x {
                            query.matches(noderef.borrow().deref())
                        } else{ true }
                    }).map(|x| x.clone()).collect();

                let mut results : Vec<HTMLEnum>= vec![];

                for child in filtered_children{
                    for elem in contents {
                        match elem {
                            MarsecElement::HTML(any) => { results.push(any.clone()); }
                            MarsecElement::Marsec(tok) => { results.append(&mut tok.apply_to(tag_invocation, vec![child.clone()])) }
                        }
                    }
                }

                return results;
            }
        }
    }
}

/// Elements, as seen by the DEFINE directives.
/// Special elements are not allowed outside of DEFINE directives
impl MarsecElement<'_>{

    /// Turn a HTML Enum (Text, Comment, Node) into a Marsec Element
    fn from_html(html: HTMLEnum) -> Result<MarsecElement, MarsecError> {

        match html{

            Node(html_ref) => {

                let html_elem = Rc::try_unwrap(html_ref).expect("Tried to unwrap multiply owned node").into_inner();

                match html_elem.name(){

                    "paste-children" => {

                        let HTMLNode{
                            attributes,
                            ..
                        } = html_elem;

                        let element_query = match attributes.iter()
                            .find(|(attr, val)| attr == &"select")
                            .map(|(attr, val)| val)
                        {
                            None => "",
                            Some(None) => { return Err(MarsecError::ValuelessQuery) }
                            Some(Some(query)) => query
                        };

                        Ok(MarsecElement::Marsec(
                            CHILDREN {
                                query: HCombinedQuery::from_str(element_query).map_err(|e| MarsecError::InvalidQuery(element_query))?
                            }
                        ))
                    }

                    "de-mux" => {

                        let HTMLNode{
                            attributes,
                            children,
                            ..
                        } = html_elem;

                        let element_query = match attributes.iter()
                            .find(|(attr, val)| attr == &"select")
                            .map(|(attr, val)| val)
                        {
                            None => "",
                            Some(None) => { return Err(MarsecError::ValuelessQuery) }
                            Some(Some(query)) => query
                        };

                        let mut new_children = vec![];

                        for child in children.into_iter() {
                            new_children.push(Self::from_html(child)?);
                        }

                        Ok(MarsecElement::Marsec(
                            DEMUX{
                                query: HCombinedQuery::from_str(element_query).map_err(|e| MarsecError::InvalidQuery(element_query))?,
                                contents: new_children
                            }
                        ))
                    }

                    _ => {
                        Ok(MarsecElement::HTML(Node(Rc::new(RefCell::from(html_elem)))))
                    }
                }
            }

            other => Ok(MarsecElement::HTML(other))
        }
    }
}

/// Reads a (top-level) HTMLToken Stream, and extracts the top-level special directives
fn filter_out_directives(stream: Vec<HTMLEnum>) -> (Vec<Directives>, Vec<HTMLEnum>) {
    let mut directives = vec![];
    let mut out = vec![];

    for element in stream {
        match element {
            Node(html_element) => {
                let raw = Rc::try_unwrap(html_element).expect("More than 1 reference").into_inner();

                if DIRECTIVE_NAMES.contains(&raw.name().to_lowercase().as_str()) {
                    directives.push(Directives::from_html(raw));
                    continue;
                } else {
                    out.push(Node(Rc::new(RefCell::from(raw))));
                }
            }
            _ => {
                out.push(element);
            }
        }
    }

    return (directives, out);
}

*/