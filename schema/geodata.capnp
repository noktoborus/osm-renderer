@0xabca199a73392829;

struct Tag {
    key @0 :Text;
    value @1 :Text;
}

struct TagList {
    tags @0 :List(Tag);
}

struct Coords {
    lat @0 :Float64;
    lon @1 :Float64;
}

struct Node {
    globalId @0 :UInt64;
    coords @1 :Coords;
    tags @2 :TagList;
}

struct Way {
    globalId @0 :UInt64;
    localNodeIds @1 :List(UInt32);
    tags @2 :TagList;
}

struct Relation {
    globalId @0 :UInt64;
    localNodeIds @1 :List(UInt32);
    localWayIds @2 :List(UInt32);
    tags @3 :TagList;
}

struct Tile {
    tileX @0 :UInt32;
    tileY @1 :UInt32;
    localNodeIds @2 :List(UInt32);
    localWayIds @3 :List(UInt32);
    localRelationIds @4 :List(UInt32);
}

struct Geodata {
    nodes @0 :List(Node);
    ways @1 :List(Way);
    relations @2 :List(Relation);
    tiles @3 :List(Tile);
}