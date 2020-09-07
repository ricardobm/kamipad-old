# kamipad-data

Database library for the Kamipad application.

At its core, Kamipad's database is a collection of textual notes and associated
metadata, with support for arbitrary N-to-N relationships between notes. Both
notes and metadata allow for arbitrary structured textual content.

The library provides the following services:

- Resilient versioned persistent storage for notes and metadata.
- Full-text and metadata indexing.
- Additional support for editing facilities (e.g. undo/redo, non-saved edits).
- Limited support for external data files (e.g. images, videos, sound, etc.).
- Support for arbitrary N-to-N relationships between notes.

## Anatomy of a Note

A `Note` is the central concept of Kamipad's database: it is an uniquely
identifiable container for all data and metadata stored in the database.

Notes are stored as human-readable textual files. Each Note has an universally
unique identifier (UUID). This ID never changes and is used everywhere to
reference that particular Note.

The Note's format is designed to allow arbitrary data and metadata to be stored
by client applications, and to support a plugin-style architecture for consuming
the data and metadata. The actual Note's content is structured as a tree
hierarchy of arbitrarily-typed textual nodes.

The library supports N-to-N relationships between Notes. Those are stored as
part of the Note's metadata, which allows different types of relationships to
be encoded.

## Storage and versioning

Each Note is stored as a human-readable text file with the Note's UUID as the
file name. A central concept of the database is that it should be possible to
reconstruct the full database just from those Note files alone—as such, Note's
files contain the entire database data and are considered the canonical source
for it.

Once a Note is written to the database it is never deleted under normal
operation. Editing a Note just creates a new version, while deleting a Note just
flags it as deleted, eventually removing it from the relationship graphs.

To avoid generating too many spurious versions, the database has the provision
of temporary edits to a Note which are then coalesced to a single version and
the temporary edits dropped. Those edits are persisted in a durable manner, but
are not visible to the main database, and need to be queried separately by
clients (they also won't change the indexes).

## Indexing

The database indexes every metadata key/value pair by value and provides a full
text index for all of the Note's content.

Additionally, relationships are also maintained as metadata and receive custom
support from the database.

The database uses a lazy approach for indexing that can produce false positives.
As such, the Note's content should always be used as the canonical source of all
data, and the indexes should just be used as a pre-filtering step to database
operations.


## External and non-textual content

Notes can only contain text, so any kind of non-textual content must be
maintained externally as a file. The database library provides some limited
support for managing those external files as generic data blobs.
