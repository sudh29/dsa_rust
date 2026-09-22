	def moveToFront(self):
		curr = self.head
		prev = None
		if not curr or not curr.next:
			return
		while curr and curr.next :
			prev = curr
			curr = curr.next
		print(curr.data,prev.data)
		prev.next = None
		curr.next = self.head
		self.head = curr
