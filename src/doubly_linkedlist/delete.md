# List delete

While deleting from a list, we need to consider two cases - is the node being deleted the last or
first node or is an inner node? The way we handle them is different. To make things simple, we have
implemented few internal helper functions. We turn to those helper functions in the next section.
