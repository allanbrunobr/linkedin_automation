searchState.loadedDescShard("bson", 0, "BSON, short for Binary JSON, is a binary-encoded …\nAlias for <code>Vec&lt;Bson&gt;</code>.\nArray\nArray\nArray\nRepresents a BSON binary value.\nBinary data\nBinary data\nBinary data\nBoolean value\nBoolean value\nBoolean value\nPossible BSON value types.\nThe legacy representation of UUIDs in BSON used by the C# …\nStruct representing a BSON datetime. Note: BSON datetimes …\nUTC datetime\nUTC datetime\nUTC datetime\nRepresents a DBPointer. (Deprecated)\nDBPointer (Deprecated)\nDBPointer (Deprecated)\nDBPointer (Deprecated)\nStruct representing a BSON Decimal128 type.\n128-bit decimal floating point\n128-bit decimal floating point\n128-bit decimal floating point\nSerde Deserializer\nOptions used to configure a <code>Deserializer</code>. These can also …\nA BSON document represented as an associative HashMap with …\nEmbedded document\nEmbedded document\nEmbedded document\n64-bit binary floating point\n64-bit binary floating point\n64-bit binary floating point\n32-bit signed integer\n32-bit signed integer\n32-bit signed integer\n64-bit signed integer\n64-bit signed integer\n64-bit signed integer\nThe legacy representation of UUIDs in BSON used by the …\nJavaScript code\nJavaScript code\nJavaScript code\nRepresents a BSON code with scope value.\nJavaScript code w/ scope\nJavaScript code w/ scope\nJavaScript code w/ scope\nMax key\nMax key\nMax key\nMin key\nMin key\nMin key\nNull value\nNull value\nNull value\nObjectId\nObjectId\nObjectId\nThe legacy representation of UUIDs in BSON used by the …\nA slice of a BSON document containing a BSON array value …\nAn owned BSON array value (akin to <code>std::path::PathBuf</code>), …\nA BSON binary value referencing raw bytes stored elsewhere.\nA BSON value backed by owned raw BSON bytes.\nA BSON value referencing raw bytes stored elsewhere.\nA BSON DB pointer value referencing raw bytes stored …\nA slice of a BSON document (akin to <code>std::str</code>). This can be …\nAn owned BSON document (akin to <code>std::path::PathBuf</code>), …\nA BSON “code with scope” value backed by owned raw …\nA BSON “code with scope” value referencing raw bytes …\nA BSON regex referencing raw bytes stored elsewhere.\nRepresents a BSON regular expression value.\nRegular expression\nRegular expression\nRegular expression\nSerde Serializer\nOptions used to configure a <code>Serializer</code>.\nThe canonical representation of UUIDs in BSON (binary with …\nUTF-8 string\nUTF-8 string\nUTF-8 string\nSymbol (Deprecated)\nSymbol (Deprecated)\nSymbol (Deprecated)\nRepresents a BSON timestamp value.\nTimestamp\nTimestamp\nTimestamp\nUndefined value (Deprecated)\nUndefined value (Deprecated)\nUndefined value (Deprecated)\nA struct modeling a BSON UUID value (i.e. a Binary value …\nEnum of the possible representations to use when …\nIf <code>self</code> is <code>Array</code>, return its value. Returns <code>None</code> otherwise.\nIf <code>self</code> is <code>Array</code>, return a mutable reference to its value. …\nIf <code>self</code> is <code>Boolean</code>, return its value. Returns <code>None</code> …\nIf <code>self</code> is <code>DateTime</code>, return its value. Returns <code>None</code> …\nIf <code>self</code> is <code>DateTime</code>, return a mutable reference to its …\nIf <code>self</code> is <code>DbPointer</code>, return its value.  Returns <code>None</code> …\nIf <code>self</code> is <code>Document</code>, return its value. Returns <code>None</code> …\nIf <code>self</code> is <code>Document</code>, return a mutable reference to its …\nIf <code>self</code> is <code>Double</code>, return its value as an <code>f64</code>. Returns <code>None</code>\nIf <code>self</code> is <code>Int32</code>, return its value. Returns <code>None</code> otherwise.\nIf <code>self</code> is <code>Int64</code>, return its value. Returns <code>None</code> otherwise.\nIf <code>self</code> is <code>Null</code>, return <code>()</code>. Returns <code>None</code> otherwise.\nIf <code>self</code> is <code>ObjectId</code>, return its value. Returns <code>None</code> …\nIf <code>self</code> is <code>ObjectId</code>, return a mutable reference to its …\nIf <code>self</code> is <code>String</code>, return its value as a <code>&amp;str</code>. Returns <code>None</code>\nIf <code>self</code> is <code>String</code>, return a mutable reference to its value …\nIf <code>self</code> is <code>Symbol</code>, return its value. Returns <code>None</code> …\nIf <code>self</code> is <code>Symbol</code>, return a mutable reference to its …\nIf <code>self</code> is <code>Timestamp</code>, return its value. Returns <code>None</code> …\nConstruct a bson::BSON value from a literal.\nThe binary bytes.\nThe binary bytes.\nThe JavaScript code.\nThe code value.\nThe JavaScript code.\nModule containing functionality related to BSON DateTimes. …\nDeserializer\nBSON Decimal128 data type representation\nDeserialize this value given this <code>Deserializer</code>.\nConstruct a bson::Document value.\nA BSON document represented as an associative HashMap with …\nGet the <code>ElementType</code> of this value.\nDeserialization and serialization of MongoDB Extended JSON …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nDeserialize a <code>T</code> from the provided <code>Bson</code> value.\nDeserialize a <code>T</code> from the provided <code>Bson</code> value, configuring …\nDeserialize a <code>T</code> from the provided <code>Document</code>.\nDeserialize a <code>T</code> from the provided <code>Document</code>, configuring …\nExamples\nDeserialize an instance of type <code>T</code> from an I/O stream of …\nDeserialize an instance of type <code>T</code> from an I/O stream of …\nDeserialize an instance of type <code>T</code> from a slice of BSON …\nDeserialize an instance of type <code>T</code> from a slice of BSON …\nWhether the <code>Deserializer</code> should present itself as human …\nWhether the <code>Serializer</code> should present itself as human …\nAn incrementing value to order timestamps with the same …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConverts the Bson value into its canonical extended JSON …\nConverts the Bson value into its relaxed extended JSON …\nModule containing functionality related to BSON ObjectIds. …\nThe options for the regex.\nThe options for the regex.\nThe regex pattern to match.\nThe regex pattern to match.\nAn API for interacting with raw BSON bytes.\nConstruct a <code>crate::RawBson</code> value from a literal.\nConstruct a <code>crate::RawDocumentBuf</code> value.\nThe scope document containing variable bindings.\nThe scope document.\nThe scope document containing variable bindings.\nSerializer\nCollection of helper functions for serializing to and …\nConstants derived from the BSON Specification Version 1.1.\nThe subtype of the bytes.\nThe subtype of the binary value.\nThe number of seconds since the Unix epoch.\nEncode a <code>T</code> Serializable into a <code>Bson</code> value.\nEncode a <code>T</code> into a <code>Bson</code> value, configuring the underlying …\nEncode a <code>T</code> Serializable into a BSON <code>Document</code>.\nEncode a <code>T</code> into a <code>Document</code>, configuring the underlying …\nSerialize the given <code>T</code> as a <code>RawDocumentBuf</code>.\nSerialize the given <code>T</code> as a BSON byte vector.\nUUID support for BSON.\nRepresents a BSON binary value.\nWhile trying to decode from base64, an error was returned.\nContains the error value\nPossible errors that can arise during <code>Binary</code> construction.\nContains the success value\nBorrow the contents as a <code>RawBinaryRef</code>.\nThe binary bytes.\nReturns the argument unchanged.\nCreates a <code>Binary</code> from a base64 string and optional …\nSerializes a <code>Uuid</code> into BSON <code>Binary</code> type\nSerializes a <code>Uuid</code> into BSON binary type and takes the …\nCalls <code>U::from(self)</code>.\nThe subtype of the bytes.\nDeserializes a BSON <code>Binary</code> type into a <code>Uuid</code> using the …\nDeserializes a BSON <code>Binary</code> type into a <code>Uuid</code> according to …\nError returned when a <code>DateTime</code> cannot be represented in a …\nStruct representing a BSON datetime. Note: BSON datetimes …\nBuilder for constructing a BSON <code>DateTime</code>\nContains the error value\nErrors that can occur during <code>DateTime</code> construction and …\nError returned when an invalid datetime format is provided …\nThe latest possible date that can be represented in BSON.\nThe earliest possible date that can be represented in BSON.\nContains the success value\nAlias for <code>Result&lt;T, DateTime::Error&gt;</code>\nConvert a builder with a specified year, month, day, and …\nReturns a builder used to construct a <code>DateTime</code> from a …\nReturns the time elapsed since <code>earlier</code>, or <code>None</code> if the …\nSets the day for the builder instance. Values in the range …\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvert the given <code>chrono::DateTime</code> into a <code>bson::DateTime</code>, …\nMakes a new <code>DateTime</code> from the number of non-leap …\nConvert the given <code>std::time::SystemTime</code> to a <code>DateTime</code>.\nSets the hour (24-hour format) for the builder instance. …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSets the millisecond for the builder instance. Values must …\nSets the minute for the builder instance. Values must be …\nSets the month for the builder instance. Maps months as …\nReturns a <code>DateTime</code> which corresponds to the current date …\nConvert the given RFC 3339 formatted string to a <code>DateTime</code>, …\nReturns the time elapsed since <code>earlier</code>, or a <code>Duration</code> of …\nSets the second for the builder instance. Values must be …\nReturns the number of non-leap-milliseconds since January …\nConvert this <code>DateTime</code> to a <code>chrono::DateTime&lt;Utc&gt;</code>.\nConvert this <code>DateTime</code> to an RFC 3339 formatted string.  …\nConvert this <code>DateTime</code> to a <code>std::time::SystemTime</code>.\nConvert this <code>DateTime</code> to an RFC 3339 formatted string.\nSets the year for the builder instance. Years between …\nA general error encountered during deserialization. See: …\nSerde Deserializer\nOptions used to configure a <code>Deserializer</code>. These can also …\nThe end of the BSON input was reached too soon.\nContains the error value\nPossible errors that can arise during decoding.\nA <code>std::string::FromUtf8Error</code> encountered while decoding a …\nA <code>std::io::Error</code> encountered while deserializing.\nContains the success value\nAlias for <code>Result&lt;T, Error&gt;</code>.\nWhile decoding a <code>Document</code> from bytes, an unexpected or …\nCreate a builder struct used to construct a …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nDeserialize a <code>T</code> from the provided <code>Bson</code> value.\nDeserialize a <code>T</code> from the provided <code>Bson</code> value, configuring …\nDeserialize a <code>T</code> from the provided <code>Document</code>.\nDeserialize a <code>T</code> from the provided <code>Document</code>, configuring …\nDeserialize an instance of type <code>T</code> from an I/O stream of …\nDeserialize an instance of type <code>T</code> from an I/O stream of …\nDeserialize an instance of type <code>T</code> from a slice of BSON …\nDeserialize an instance of type <code>T</code> from a slice of BSON …\nWhether the <code>Deserializer</code> should present itself as human …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstruct a new <code>Deserializer</code> using the default options.\nCreate a new <code>Deserializer</code> using the provided options.\nThe encountered element type.\nThe key at which an unexpected/unsupported element type …\nA message describing the error.\nStruct representing a BSON Decimal128 type.\nReturns the raw byte representation of this <code>Decimal128</code>.\nReturns the argument unchanged.\nConstructs a new <code>Decimal128</code> from the provided raw byte …\nCalls <code>U::from(self)</code>.\nA BSON document represented as an associative HashMap with …\nA view into a single entry in a map, which may either be …\nContains the error value\nAn iterator over Document entries.\nAn owning iterator over Document entries.\nAn iterator over a <code>Document</code>’s keys and mutable values.\nAn iterator over an Document’s keys.\nCannot find the expected field with the specified key\nAn occupied entry.\nA view into an occupied entry in a Document. It is part of …\nContains the success value\nFound a Bson value with the specified key, but not with …\nA vacant entry.\nA view into a vacant entry in a Document. It is part of …\nError to indicate that either a value was empty or it …\nResult of accessing Bson value\nAn iterator over an Document’s values.\nClears the document, removing all values.\nReturns true if the map contains a value for the specified …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nAttempts to deserialize a <code>Document</code> from a byte stream.\nAttempt to deserialize a <code>Document</code> that may contain invalid …\nReturns a reference to the Bson corresponding to the key.\nGet a reference to an array for this key if it exists and …\nGet a mutable reference to an array for this key if it …\nGet a reference to a generic binary value for this key if …\nGet a mutable reference generic binary value for this key …\nGet a bool value for this key if it exists and has the …\nGet a mutable reference to a bool value for this key if it …\nGet a reference to a UTC datetime value for this key if it …\nGet a mutable reference to a UTC datetime value for this …\nGet a reference to a Decimal128 value for key, if it …\nGet a mutable reference to a Decimal128 value for key, if …\nGet a reference to a document for this key if it exists …\nGet a mutable reference to a document for this key if it …\nGet a floating point value for this key if it exists and …\nGet a mutable reference to a floating point value for this …\nGet an i32 value for this key if it exists and has the …\nGet a mutable reference to an i32 value for this key if it …\nGet an i64 value for this key if it exists and has the …\nGet a mutable reference to an i64 value for this key if it …\nGets a mutable reference to the Bson corresponding to the …\nGet an object id value for this key if it exists and has …\nGet a mutable reference to an object id value for this key …\nGet a string slice this key if it exists and has the …\nGet a mutable string slice this key if it exists and has …\nGet a time stamp value for this key if it exists and has …\nGet a mutable reference to a time stamp value for this key …\nSets the value of the entry with the OccupiedEntry’s key,\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns true if the document contains no elements\nReturns wether this key has a null value\nGets an iterator over the entries of the map.\nGets an iterator over pairs of keys and mutable values.\nReturns a reference to this entry’s key.\nGets a reference to the key in the entry.\nGets a collection of all keys in the document.\nReturns the number of elements in the document.\nCreates a new empty Document.\nInserts the given default value in the entry if it is …\nInserts the result of the <code>default</code> function in the entry if …\nTakes the value of the entry out of the document, and …\nAttempts to serialize the <code>Document</code> into a byte stream.\nGets a collection of all values in the document.\nDeserializing MongoDB Extended JSON v2\nA general error encountered during deserialization. See: …\nContains the error value\nError cases that can occur during deserialization from …\nErrors that can occur during OID construction and …\nContains the success value\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nContains the error value\nErrors that can occur during <code>ObjectId</code> construction and …\nAn invalid character was found in the provided hex string. …\nAn <code>ObjectId</code>’s hex string representation must be an …\nA wrapper around a raw 12-byte ObjectId.\nContains the success value\nAlias for Result&lt;T, oid::Error&gt;.\nReturns the raw byte representation of an ObjectId.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstructs a new ObjectId wrapper around the raw byte …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nGenerates a new <code>ObjectId</code>, represented in bytes. See the …\nCreates an ObjectID using a 12-byte (24-char) hexadecimal …\nRetrieves the timestamp from an <code>ObjectId</code>.\nConvert this <code>ObjectId</code> to its hex string representation.\nArray\nArray\nBinary data\nBinary data\nBoolean value\nBoolean value\nUTC datetime\nUTC datetime\nDBPointer (Deprecated)\nDBPointer (Deprecated)\n128-bit decimal floating point\n128-bit decimal floating point\nEmbedded document\nEmbedded document\n64-bit binary floating point\n64-bit binary floating point\nContains the error value\nContains the error value\nAn error that occurs when attempting to parse raw BSON …\nThe different categories of errors that can be returned …\n32-bit signed integer\n32-bit signed integer\n64-bit signed integer\n64-bit signed integer\nAn error was encountered attempting to decode the document.\nJavaScript code\nJavaScript code\nJavaScript code w/ scope\nJavaScript code w/ scope\nA BSON value did not fit the proper format.\nMax key\nMax key\nMin key\nMin key\nCannot find the expected field with the specified key\nNull value\nNull value\nObjectId\nObjectId\nContains the success value\nContains the success value\nA slice of a BSON document containing a BSON array value …\nAn owned BSON array value (akin to <code>std::path::PathBuf</code>), …\nAn iterator over borrowed raw BSON array values.\nA BSON binary value referencing raw bytes stored elsewhere.\nA BSON value backed by owned raw BSON bytes.\nA BSON value referencing raw bytes stored elsewhere.\nA BSON DB pointer value referencing raw bytes stored …\nA slice of a BSON document (akin to <code>std::str</code>). This can be …\nAn owned BSON document (akin to <code>std::path::PathBuf</code>), …\nAn iterator over the document’s elements.\nA BSON “code with scope” value backed by owned raw …\nA BSON “code with scope” value referencing raw bytes …\nA BSON regex referencing raw bytes stored elsewhere.\nRegular expression\nRegular expression\nUTF-8 string\nUTF-8 string\nSymbol (Deprecated)\nSymbol (Deprecated)\nTimestamp\nTimestamp\nUndefined value (Deprecated)\nUndefined value (Deprecated)\nFound a Bson value with the specified key, but not with …\nImproper UTF-8 bytes were found when proper UTF-8 was …\nError to indicate that either a value was empty or it …\nThe type of error encountered when using a direct getter …\nAppend a key value pair to the end of the document without …\nAppend a key value pair to the end of the document without …\nGets a reference to the <code>RawArrayBuf</code> that’s wrapped or …\nGets the <code>RawArray</code> that’s referenced or returns <code>None</code> if …\nGets a mutable reference to the <code>RawArrayBuf</code> that’s …\nGets a reference to the <code>Binary</code> that’s wrapped or returns …\nGets the <code>RawBinaryRef</code> that’s referenced or returns <code>None</code> …\nGets the wrapped <code>bool</code> value or returns <code>None</code> if the wrapped …\nGets the <code>bool</code> that’s referenced or returns <code>None</code> if the …\nGets a reference to the raw bytes of the <code>RawArray</code>.\nReturn a reference to the contained data as a <code>&amp;[u8]</code>\nGets the wrapped <code>crate::DateTime</code> value or returns <code>None</code> if …\nGets the <code>crate::DateTime</code> that’s referenced or returns …\nGets a reference to the <code>crate::DbPointer</code> that’s wrapped …\nGets the <code>RawDbPointerRef</code> that’s referenced or returns …\nGets a reference to the <code>RawDocumentBuf</code> that’s wrapped or …\nGets the <code>RawDocument</code> that’s referenced or returns <code>None</code> …\nGets a mutable reference to the <code>RawDocumentBuf</code> that’s …\nGets the wrapped <code>f64</code> value or returns <code>None</code> if the value isn…\nGets the <code>f64</code> that’s referenced or returns <code>None</code> if the …\nGets the wrapped <code>i32</code> value or returns <code>None</code> if the wrapped …\nGets the <code>i32</code> that’s referenced or returns <code>None</code> if the …\nGets the wrapped <code>i64</code> value or returns <code>None</code> if the wrapped …\nGets the <code>i64</code> that’s referenced or returns <code>None</code> if the …\nGets a reference to the code that’s wrapped or returns …\nGets the code that’s referenced or returns <code>None</code> if the …\nGets a reference to the <code>RawJavaScriptCodeWithScope</code> that’…\nGets the <code>RawJavaScriptCodeWithScope</code> that’s referenced or …\nReturns <code>Some(())</code> if this value is null, otherwise returns …\nGets the null value that’s referenced or returns <code>None</code> if …\nGets the wrapped <code>crate::oid::ObjectId</code> value or returns <code>None</code>…\nGets the <code>crate::oid::ObjectId</code> that’s referenced or …\nGets a <code>RawBsonRef</code> value referencing this owned raw BSON …\nGets a reference to the <code>Regex</code> that’s wrapped or returns …\nGets the <code>RawRegexRef</code> that’s referenced or returns <code>None</code> …\nGets a reference to the <code>String</code> that’s wrapped or returns …\nGets the <code>&amp;str</code> that’s referenced or returns <code>None</code> if the …\nGets a reference to the symbol that’s wrapped or returns …\nGets the symbol that’s referenced or returns <code>None</code> if the …\nGets the wrapped <code>crate::Timestamp</code> value or returns <code>None</code> if …\nGets the <code>crate::Timestamp</code> that’s referenced or returns …\nThe binary bytes.\nThe code value.\nThe JavaScript code.\nGet the <code>ElementType</code> of this value.\nGet the <code>ElementType</code> of this value.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstructs a new <code>RawDocument</code>, validating <em>only</em> the …\nConstructs a new <code>RawDocumentBuf</code>, validating <em>only</em> the …\nCreate a <code>RawDocumentBuf</code> from a <code>Document</code>.\nGets a reference to the value at the given index.\nGets a reference to the value corresponding to the given …\nGets a reference to the array at the given index or …\nGets a reference to the array value corresponding to a …\nGets a reference to the BSON binary value at the given …\nGets a reference to the BSON binary value corresponding to …\nGets the boolean at the given index or returns an error if …\nGets a reference to the boolean value corresponding to a …\nGets the DateTime at the given index or returns an error …\nGets a reference to the BSON DateTime value corresponding …\nGets a reference to the document at the given index or …\nGets a reference to the document value corresponding to a …\nGets the BSON double at the given index or returns an …\nGets a reference to the BSON double value corresponding to …\nGets the BSON int32 at the given index or returns an error …\nGets a reference to the BSON int32 value corresponding to …\nGets BSON int64 at the given index or returns an error if …\nGets a reference to the BSON int64 value corresponding to …\nGets the ObjectId at the given index or returns an error …\nGets a reference to the ObjectId value corresponding to a …\nGets a reference to the BSON regex at the given index or …\nGets a reference to the BSON regex value corresponding to …\nGets a reference to the string at the given index or …\nGets a reference to the string value corresponding to a …\nGets a reference to the BSON timestamp at the given index …\nGets a reference to the BSON timestamp value corresponding …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturn the contained data as a <code>Vec&lt;u8&gt;</code>\nWhether this array contains any elements or not.\nReturns whether this document contains any elements or not.\nGets an iterator over the elements in the <code>RawDocument</code> that …\nGets an iterator over the elements in the <code>RawDocumentBuf</code>, …\nGets an iterator over the elements in the <code>RawDocument</code>, …\nGets an iterator over the elements in the <code>RawDocumentBuf</code>, …\nThe key at which the error was encountered, if any.\nThe key at which the error was encountered.\nThe type of error that was encountered.\nThe type of error that was encountered.\nConstruct a new, empty <code>RawArrayBuf</code>.\nCreates a new, empty <code>RawDocumentBuf</code>.\nThe options for the regex.\nThe regex pattern to match.\nAppend a value to the end of the array.\nThe scope document.\nThe scope document containing variable bindings.\nThe subtype of the binary value.\nCopy the contents into a <code>Binary</code>.\nConvert this <code>RawDocumentBuf</code> to a <code>Document</code>, returning an …\nConvert this borrowed <code>RawArray</code> into an owned <code>RawArrayBuf</code>.\nConvert this <code>RawBsonRef</code> to the equivalent <code>RawBson</code>.\nCreates a new <code>RawDocumentBuf</code> with an owned copy of the …\nThe actual type that was encountered.\nThe type that was expected.\nContains the error value\nPossible errors that can arise during encoding.\nAn invalid string was specified.\nA key could not be serialized to a BSON string.\nA <code>std::io::Error</code> encountered while serializing.\nContains the success value\nAlias for <code>Result&lt;T, Error&gt;</code>.\nA general error that occurred during serialization. See: …\nSerde Serializer\nOptions used to configure a <code>Serializer</code>.\nAn unsigned integer type could not fit into a signed …\nCreate a builder used to construct a new <code>SerializerOptions</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nWhether the <code>Serializer</code> should present itself as human …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstruct a new <code>Serializer</code>.\nConstruct a new <code>Serializer</code> configured with the provided …\nEncode a <code>T</code> Serializable into a <code>Bson</code> value.\nEncode a <code>T</code> into a <code>Bson</code> value, configuring the underlying …\nEncode a <code>T</code> Serializable into a BSON <code>Document</code>.\nEncode a <code>T</code> into a <code>Document</code>, configuring the underlying …\nSerialize the given <code>T</code> as a <code>RawDocumentBuf</code>.\nSerialize the given <code>T</code> as a BSON byte vector.\nA message describing the error.\nWrapping a type in <code>HumanReadable</code> signals to the BSON serde …\nContains functions to serialize a <code>crate::DateTime</code> as an …\nContains functions to serialize a <code>chrono::DateTime</code> as a …\nDeserializes a <code>crate::DateTime</code> from an RFC 3339 formatted …\nDeserializes a <code>chrono::DateTime</code> from a <code>crate::DateTime</code>.\nDeserializes a hex string from an ObjectId.\nDeserializes a i64 integer from a DateTime.\nDeserializes an ISO string from a DateTime.\nDeserializes a bson::Timestamp from a u32.\nDeserializes a u32 from an f64 (BSON double). Errors if an …\nDeserializes a u32 from a bson::Timestamp.\nDeserializes a u64 from an f64 (BSON double). Errors if an …\nReturns the argument unchanged.\nContains functions to serialize a hex string as an …\nContains functions to <code>serialize</code> a <code>i64</code> integer as <code>DateTime</code> …\nCalls <code>U::from(self)</code>.\nContains functions to serialize an RFC 3339 (ISO 8601) …\nSerializes a <code>crate::DateTime</code> as an RFC 3339 (ISO 8601) …\nSerializes a <code>chrono::DateTime</code> as a <code>crate::DateTime</code>.\nSerializes a hex string as an ObjectId.\nSerializes a i64 integer as a DateTime.\nSerializes an <code>ObjectId</code> as a hex string.\nSerializes an ISO string as a DateTime.\nSerializes a bson::Timestamp as a u32. Returns an error if …\nSerializes a u32 as an f64 (BSON double).\nAttempts to serialize a u32 as an i32. Errors if an exact …\nSerializes a u32 as an i64.\nSerializes a u32 as a bson::Timestamp.\nSerializes a u64 as an f64 (BSON double). Errors if an …\nAttempts to serialize a u64 as an i32. Errors if an exact …\nAttempts to serialize a u64 as an i64. Errors if an exact …\nContains functions to serialize a bson::Timestamp as a u32 …\nContains functions to serialize a u32 as an f64 (BSON …\nContains functions to serialize a u32 as a bson::Timestamp …\nContains functions to serialize a u64 as an f64 (BSON …\nDeserializes a <code>crate::DateTime</code> from an RFC 3339 formatted …\nSerializes a <code>crate::DateTime</code> as an RFC 3339 (ISO 8601) …\nDeserializes a <code>chrono::DateTime</code> from a <code>crate::DateTime</code>.\nSerializes a <code>chrono::DateTime</code> as a <code>crate::DateTime</code>.\nDeserializes a hex string from an ObjectId.\nSerializes a hex string as an ObjectId.\nDeserializes a i64 integer from a DateTime.\nSerializes a i64 integer as a DateTime.\nDeserializes an ISO string from a DateTime.\nSerializes an ISO string as a DateTime.\nDeserializes a bson::Timestamp from a u32.\nSerializes a bson::Timestamp as a u32. Returns an error if …\nDeserializes a u32 from an f64 (BSON double). Errors if an …\nSerializes a u32 as an f64 (BSON double).\nDeserializes a u32 from a bson::Timestamp.\nSerializes a u32 as a bson::Timestamp.\nDeserializes a u64 from an f64 (BSON double). Errors if an …\nSerializes a u64 as an f64 (BSON double). Errors if an …\nArray\nBinary data\nThe available binary subtypes, plus a user-defined slot.\nBool value\nUTC datetime\nDeprecated.\n128-bit decimal floating point\n64-bit binary floating point\nAll available BSON element types.\nEmbedded document\n32-bit integer\n64-bit integer\nJavaScript code\nJavaScript code w/ scope\nNull value\nObjectId\nRegular expression - The first cstring is the regex …\nUTF-8 string\nDeprecated.\nTimestamp\nDeprecated. Undefined (value)\nAttempt to convert from a <code>u8</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe legacy representation of UUIDs in BSON used by the C# …\nContains the error value\nErrors that can occur during <code>Uuid</code> construction and …\nError returned from <code>crate::Binary::to_uuid</code> if the …\nError returned when an invalid string is provided to …\nThe legacy representation of UUIDs in BSON used by the …\nContains the success value\nThe legacy representation of UUIDs in BSON used by the …\nError returned when the representation specified does not …\nAlias for <code>Result&lt;T, bson::uuid::Error&gt;</code>.\nThe canonical representation of UUIDs in BSON (binary with …\nA struct modeling a BSON UUID value (i.e. a Binary value …\nEnum of the possible representations to use when …\nReturns an array of 16 bytes containing the <code>Uuid</code>’s data.\nReturns the argument unchanged.\nCreates a <code>Uuid</code> using the supplied big-endian bytes.\nCalls <code>U::from(self)</code>.\nCreates a random UUID.\nCreates a <code>Uuid</code> from the provided hex string.\nThe actual subtype of the binary value.\nThe subtype that was expected given the requested …\nThe actual length of the data.\nThe requested representation.")