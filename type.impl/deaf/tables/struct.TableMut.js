(function() {
    var type_impls = Object.fromEntries([["deaf",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TableMut%3C'a,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#160-241\">Source</a><a href=\"#impl-TableMut%3C'a,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T&gt; <a class=\"struct\" href=\"deaf/tables/struct.TableMut.html\" title=\"struct deaf::tables::TableMut\">TableMut</a>&lt;'a, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"deaf/tables/trait.TableItem.html\" title=\"trait deaf::tables::TableItem\">TableItem</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.append\" class=\"method\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#198-200\">Source</a><h4 class=\"code-header\">pub fn <a href=\"deaf/tables/struct.TableMut.html#tymethod.append\" class=\"fn\">append</a>(&amp;mut self, item: T) -&gt; <a class=\"type\" href=\"deaf/errors/type.Result.html\" title=\"type deaf::errors::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Append an item to the table</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.prepend\" class=\"method\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#203-205\">Source</a><h4 class=\"code-header\">pub fn <a href=\"deaf/tables/struct.TableMut.html#tymethod.prepend\" class=\"fn\">prepend</a>(&amp;mut self, item: T) -&gt; <a class=\"type\" href=\"deaf/errors/type.Result.html\" title=\"type deaf::errors::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Prepend an item to the table</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.insert\" class=\"method\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#208-225\">Source</a><h4 class=\"code-header\">pub fn <a href=\"deaf/tables/struct.TableMut.html#tymethod.insert\" class=\"fn\">insert</a>(&amp;mut self, index: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a>, item: T) -&gt; <a class=\"type\" href=\"deaf/errors/type.Result.html\" title=\"type deaf::errors::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Insert an item into the table</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.remove\" class=\"method\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#228-239\">Source</a><h4 class=\"code-header\">pub fn <a href=\"deaf/tables/struct.TableMut.html#tymethod.remove\" class=\"fn\">remove</a>(&amp;mut self, index: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"type\" href=\"deaf/errors/type.Result.html\" title=\"type deaf::errors::Result\">Result</a>&lt;T&gt;</h4></section></summary><div class=\"docblock\"><p>Remove an item from the table by index</p>\n</div></details></div></details>",0,"deaf::tables::table::ArrayMut","deaf::tables::table::SymbolTableMut","deaf::tables::table::RelTableMut","deaf::tables::table::RelaTableMut","deaf::tables::table::StringTableMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TableView%3CT%3E-for-TableMut%3C'a,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#243-250\">Source</a><a href=\"#impl-TableView%3CT%3E-for-TableMut%3C'a,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T&gt; <a class=\"trait\" href=\"deaf/tables/trait.TableView.html\" title=\"trait deaf::tables::TableView\">TableView</a>&lt;T&gt; for <a class=\"struct\" href=\"deaf/tables/struct.TableMut.html\" title=\"struct deaf::tables::TableMut\">TableMut</a>&lt;'a, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"deaf/tables/trait.TableItem.html\" title=\"trait deaf::tables::TableItem\">TableItem</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.section\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#247-249\">Source</a><a href=\"#method.section\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#tymethod.section\" class=\"fn\">section</a>(&amp;self) -&gt; &amp;<a class=\"struct\" href=\"deaf/struct.Section.html\" title=\"struct deaf::Section\">Section</a></h4></section></summary><div class='docblock'>Get an immutable reference to the internal section</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.name_index\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#32-34\">Source</a><a href=\"#method.name_index\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#method.name_index\" class=\"fn\">name_index</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>Get the name index of the internal section</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.iterator\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#37-42\">Source</a><a href=\"#method.iterator\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#method.iterator\" class=\"fn\">iterator</a>(&amp;self) -&gt; <a class=\"struct\" href=\"deaf/common/struct.ByteIter.html\" title=\"struct deaf::common::ByteIter\">ByteIter</a>&lt;'_&gt; <a href=\"#\" class=\"tooltip\" data-notable-ty=\"ByteIter&lt;&#39;_&gt;\">ⓘ</a></h4></section></summary><div class='docblock'>Get an iterator over each item’s binary data</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.data\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#45-49\">Source</a><a href=\"#method.data\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#method.data\" class=\"fn\">data</a>(&amp;self, index: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"type\" href=\"deaf/errors/type.Result.html\" title=\"type deaf::errors::Result\">Result</a>&lt;&amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u8.html\">u8</a>]&gt;</h4></section></summary><div class='docblock'>Get a slice of data that represents an item</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.offset\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#52-61\">Source</a><a href=\"#method.offset\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#method.offset\" class=\"fn\">offset</a>(&amp;self, index: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>Get the offset of an item from the index</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.at\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#64-66\">Source</a><a href=\"#method.at\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#method.at\" class=\"fn\">at</a>(&amp;self, index: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"type\" href=\"deaf/errors/type.Result.html\" title=\"type deaf::errors::Result\">Result</a>&lt;T&gt;</h4></section></summary><div class='docblock'>Get an element from the table</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.at_offset\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#69-75\">Source</a><a href=\"#method.at_offset\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#method.at_offset\" class=\"fn\">at_offset</a>(&amp;self, offset: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"type\" href=\"deaf/errors/type.Result.html\" title=\"type deaf::errors::Result\">Result</a>&lt;T&gt;</h4></section></summary><div class='docblock'>Get an element from the table at a byte offset</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.items\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#78-82\">Source</a><a href=\"#method.items\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#method.items\" class=\"fn\">items</a>(&amp;self) -&gt; <a class=\"type\" href=\"deaf/errors/type.Result.html\" title=\"type deaf::errors::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;T&gt;&gt;</h4></section></summary><div class='docblock'>Get all items from the table</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.len\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#85-91\">Source</a><a href=\"#method.len\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#method.len\" class=\"fn\">len</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>Get the number of items in the table</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.size\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#94-96\">Source</a><a href=\"#method.size\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#method.size\" class=\"fn\">size</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>Get the number of bytes in the table</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.layout\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#99-101\">Source</a><a href=\"#method.layout\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#method.layout\" class=\"fn\">layout</a>(&amp;self) -&gt; <a class=\"enum\" href=\"deaf/common/enum.Layout.html\" title=\"enum deaf::common::Layout\">Layout</a></h4></section></summary><div class='docblock'>Get the layout being used by this table</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.width\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#104-106\">Source</a><a href=\"#method.width\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#method.width\" class=\"fn\">width</a>(&amp;self) -&gt; <a class=\"enum\" href=\"deaf/common/enum.Width.html\" title=\"enum deaf::common::Width\">Width</a></h4></section></summary><div class='docblock'>Get the width being used by this table</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.has_fixed_size\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#109-111\">Source</a><a href=\"#method.has_fixed_size\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#method.has_fixed_size\" class=\"fn\">has_fixed_size</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>True if items are all the same size</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.has_variable_size\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#114-116\">Source</a><a href=\"#method.has_variable_size\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"deaf/tables/trait.TableView.html#method.has_variable_size\" class=\"fn\">has_variable_size</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>True if items can be different sizes</div></details></div></details>","TableView<T>","deaf::tables::table::ArrayMut","deaf::tables::table::SymbolTableMut","deaf::tables::table::RelTableMut","deaf::tables::table::RelaTableMut","deaf::tables::table::StringTableMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TryFrom%3C%26mut+Section%3E-for-TableMut%3C'a,+ArrayItem%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#362-372\">Source</a><a href=\"#impl-TryFrom%3C%26mut+Section%3E-for-TableMut%3C'a,+ArrayItem%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'a mut <a class=\"struct\" href=\"deaf/struct.Section.html\" title=\"struct deaf::Section\">Section</a>&gt; for <a class=\"struct\" href=\"deaf/tables/struct.TableMut.html\" title=\"struct deaf::tables::TableMut\">TableMut</a>&lt;'a, <a class=\"struct\" href=\"deaf/tables/struct.ArrayItem.html\" title=\"struct deaf::tables::ArrayItem\">ArrayItem</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#364\">Source</a><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = <a class=\"enum\" href=\"deaf/errors/enum.Error.html\" title=\"enum deaf::errors::Error\">Error</a></h4></section></summary><div class='docblock'>The type returned in the event of a conversion error.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#366-371\">Source</a><a href=\"#method.try_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#tymethod.try_from\" class=\"fn\">try_from</a>(section: &amp;'a mut <a class=\"struct\" href=\"deaf/struct.Section.html\" title=\"struct deaf::Section\">Section</a>) -&gt; <a class=\"type\" href=\"deaf/errors/type.Result.html\" title=\"type deaf::errors::Result\">Result</a>&lt;Self&gt;</h4></section></summary><div class='docblock'>Performs the conversion.</div></details></div></details>","TryFrom<&'a mut Section>","deaf::tables::table::ArrayMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TryFrom%3C%26mut+Section%3E-for-TableMut%3C'a,+RelItem%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#338-348\">Source</a><a href=\"#impl-TryFrom%3C%26mut+Section%3E-for-TableMut%3C'a,+RelItem%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'a mut <a class=\"struct\" href=\"deaf/struct.Section.html\" title=\"struct deaf::Section\">Section</a>&gt; for <a class=\"struct\" href=\"deaf/tables/struct.TableMut.html\" title=\"struct deaf::tables::TableMut\">TableMut</a>&lt;'a, <a class=\"struct\" href=\"deaf/tables/struct.RelItem.html\" title=\"struct deaf::tables::RelItem\">RelItem</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#340\">Source</a><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = <a class=\"enum\" href=\"deaf/errors/enum.Error.html\" title=\"enum deaf::errors::Error\">Error</a></h4></section></summary><div class='docblock'>The type returned in the event of a conversion error.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#342-347\">Source</a><a href=\"#method.try_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#tymethod.try_from\" class=\"fn\">try_from</a>(section: &amp;'a mut <a class=\"struct\" href=\"deaf/struct.Section.html\" title=\"struct deaf::Section\">Section</a>) -&gt; <a class=\"type\" href=\"deaf/errors/type.Result.html\" title=\"type deaf::errors::Result\">Result</a>&lt;Self&gt;</h4></section></summary><div class='docblock'>Performs the conversion.</div></details></div></details>","TryFrom<&'a mut Section>","deaf::tables::table::RelTableMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TryFrom%3C%26mut+Section%3E-for-TableMut%3C'a,+RelaItem%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#314-324\">Source</a><a href=\"#impl-TryFrom%3C%26mut+Section%3E-for-TableMut%3C'a,+RelaItem%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'a mut <a class=\"struct\" href=\"deaf/struct.Section.html\" title=\"struct deaf::Section\">Section</a>&gt; for <a class=\"struct\" href=\"deaf/tables/struct.TableMut.html\" title=\"struct deaf::tables::TableMut\">TableMut</a>&lt;'a, <a class=\"struct\" href=\"deaf/tables/struct.RelaItem.html\" title=\"struct deaf::tables::RelaItem\">RelaItem</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#316\">Source</a><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = <a class=\"enum\" href=\"deaf/errors/enum.Error.html\" title=\"enum deaf::errors::Error\">Error</a></h4></section></summary><div class='docblock'>The type returned in the event of a conversion error.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#318-323\">Source</a><a href=\"#method.try_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#tymethod.try_from\" class=\"fn\">try_from</a>(section: &amp;'a mut <a class=\"struct\" href=\"deaf/struct.Section.html\" title=\"struct deaf::Section\">Section</a>) -&gt; <a class=\"type\" href=\"deaf/errors/type.Result.html\" title=\"type deaf::errors::Result\">Result</a>&lt;Self&gt;</h4></section></summary><div class='docblock'>Performs the conversion.</div></details></div></details>","TryFrom<&'a mut Section>","deaf::tables::table::RelaTableMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TryFrom%3C%26mut+Section%3E-for-TableMut%3C'a,+StringItem%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#290-300\">Source</a><a href=\"#impl-TryFrom%3C%26mut+Section%3E-for-TableMut%3C'a,+StringItem%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'a mut <a class=\"struct\" href=\"deaf/struct.Section.html\" title=\"struct deaf::Section\">Section</a>&gt; for <a class=\"struct\" href=\"deaf/tables/struct.TableMut.html\" title=\"struct deaf::tables::TableMut\">TableMut</a>&lt;'a, <a class=\"struct\" href=\"deaf/tables/struct.StringItem.html\" title=\"struct deaf::tables::StringItem\">StringItem</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#292\">Source</a><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = <a class=\"enum\" href=\"deaf/errors/enum.Error.html\" title=\"enum deaf::errors::Error\">Error</a></h4></section></summary><div class='docblock'>The type returned in the event of a conversion error.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#294-299\">Source</a><a href=\"#method.try_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#tymethod.try_from\" class=\"fn\">try_from</a>(section: &amp;'a mut <a class=\"struct\" href=\"deaf/struct.Section.html\" title=\"struct deaf::Section\">Section</a>) -&gt; <a class=\"type\" href=\"deaf/errors/type.Result.html\" title=\"type deaf::errors::Result\">Result</a>&lt;Self&gt;</h4></section></summary><div class='docblock'>Performs the conversion.</div></details></div></details>","TryFrom<&'a mut Section>","deaf::tables::table::StringTableMut"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TryFrom%3C%26mut+Section%3E-for-TableMut%3C'a,+Symbol%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#265-276\">Source</a><a href=\"#impl-TryFrom%3C%26mut+Section%3E-for-TableMut%3C'a,+Symbol%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'a mut <a class=\"struct\" href=\"deaf/struct.Section.html\" title=\"struct deaf::Section\">Section</a>&gt; for <a class=\"struct\" href=\"deaf/tables/struct.TableMut.html\" title=\"struct deaf::tables::TableMut\">TableMut</a>&lt;'a, Symbol&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#267\">Source</a><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = <a class=\"enum\" href=\"deaf/errors/enum.Error.html\" title=\"enum deaf::errors::Error\">Error</a></h4></section></summary><div class='docblock'>The type returned in the event of a conversion error.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/deaf/tables/table.rs.html#269-275\">Source</a><a href=\"#method.try_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/convert/trait.TryFrom.html#tymethod.try_from\" class=\"fn\">try_from</a>(section: &amp;'a mut <a class=\"struct\" href=\"deaf/struct.Section.html\" title=\"struct deaf::Section\">Section</a>) -&gt; <a class=\"type\" href=\"deaf/errors/type.Result.html\" title=\"type deaf::errors::Result\">Result</a>&lt;Self&gt;</h4></section></summary><div class='docblock'>Performs the conversion.</div></details></div></details>","TryFrom<&'a mut Section>","deaf::tables::table::SymbolTableMut"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[25576]}