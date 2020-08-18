#!/usr/bin/env python
import pika
import uuid
import json

class FibonacciRpcClient(object):

    def __init__(self):
        self.connection = pika.BlockingConnection(
            pika.ConnectionParameters(host='localhost'))

        self.channel = self.connection.channel()

        result = self.channel.queue_declare(queue='', exclusive=True)
        self.callback_queue = result.method.queue

        self.channel.basic_consume(
            queue=self.callback_queue,
            on_message_callback=self.on_response,
            auto_ack=True)

    def on_response(self, ch, method, props, body):
        if self.corr_id == props.correlation_id:
            self.response = body

    def call(self, n):
        self.response = None
        self.corr_id = str(uuid.uuid4())
        self.channel.basic_publish(
            exchange='',
            routing_key='rpc_queue',
            properties=pika.BasicProperties(
                reply_to=self.callback_queue,
                correlation_id=self.corr_id,
            ),
            body=str(n))
        while self.response is None:
            self.connection.process_data_events()
        return (self.response)


fibonacci_rpc = FibonacciRpcClient()

n = 3000
 
x = {
  "ground": [0.5, 0.5, 0.5],
  "sky_color": [0.7, 0.5, 1.8],
  "sphere1_center": [0.0, 1.0, 0.0],
  "sphere1_radius": 1.0,
  "sphere2_center": [-1.8, 1.0, 0.0],
  "sphere2_color": [0.4, 0.2, 0.1],
  "sphere2_radius": 1.0,
  "sphere3_center": [-1.8, 1.0, 0.0],
  "sphere3_color": [1.8, 1.0, 0.0],
  "sphere3_radius": 1.0,
  "lookfrom": [5.0, 10.0, 10.0],
  "lookat": [0.0, 0.0, 0.0],
  "vup": [0.0, 1.0, 0.0],
}

y = json.dumps(x)

response = fibonacci_rpc.call(y)
response = response.decode("utf-8")
response = response.replace('\\n', '\n')
print(response)