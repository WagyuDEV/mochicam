import queue

class SubScrub:
    _topics = None

    def __init__(self, topics=None):
        if topics is not None:
            for topic in topics:
                self.add_topic(topic)
        else:
            self._topics = []

    def _exists(self, topic):
        for tp in self._topics:
            if tp == topic:
                return True
        return False

    def add_topic(self, topic):
        if not self._exists(topic):
            self._topics.append(topic)

    def del_topic(self, topic):
        if self._exists(topic):
            self._topics.remove(topic)
    
    def publish(self, topic, data):
        return (topic, data)