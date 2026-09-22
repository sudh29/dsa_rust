# LinkList


class Node:
    def __init__(self, data):
        self.data = data
        self.next = None


class LinkList:
    def __init__(self):
        self.head = None

    # add value in front
    def push(self, newData):
        newNode = Node(newData)
        newNode.next = self.head
        self.head = newNode

    # add value in last
    def append(self, newData):
        newNode = Node(newData)

        if self.head is None:
            self.head = newNode
            return

        current = self.head
        while current.next:
            current = current.next
        current.next = newNode

    # add value after previous node
    def insertAt(self, prevNode, newData):
        if prevNode is None:
            print("No previous Node")
            return
        newNode = Node(newData)
        newNode.next = prevNode.next
        prevNode.next = newNode

    # print funtion

    def printList(self):
        current = self.head
        while current:
            print(current.data, end=" ")
            current = current.next

    # delete the list
    def deleteList(self):
        current = self.head
        while current:
            del current.data
            current = current.next

    # delete Node
    def deleteNode(self, data):
        current = self.head
        if current is not None:
            if current.data == data:
                self.head = current.next
                current = None
                return

        while current:
            if current.data == data:
                break
            prev = current
            current = current.next
        if current == None:
            return
        prev.next = current.next
        current = None


if __name__ == "__main__":
    L_list = LinkList()
    L_list.append(6)
    L_list.push(7)
    L_list.push(1)
    L_list.append(4)
    L_list.append(11)
    L_list.insertAt(L_list.head.next.next, 8)

    print("Link List")
    L_list.printList()
    print()
    L_list.deleteNode(4)
    print("Link List")
    L_list.printList()
    print()
    L_list.deleteList()
