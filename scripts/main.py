import rclpy
from rclpy.node import Node

from geometry_msgs.msg import TransformStamped
from tf2_ros import TransformBroadcaster


def tf_publisher():
    rclpy.init()
    node = Node("transform_node")
    tf_pub = TransformBroadcaster(node)

    t = TransformStamped()
    t.header.stamp = node.get_clock().now().to_msg()
    # t.header.frame_id = 'odom'
    # t.child_frame_id = 'base_link'
    t.header.frame_id = 'base_link'
    t.child_frame_id = 'odom'
    t.transform.translation.x = 0.0
    t.transform.translation.y = 2.0
    t.transform.translation.z = 0.0
    t.transform.rotation.x = 0.0
    t.transform.rotation.y = 0.0
    t.transform.rotation.z = 0.0
    t.transform.rotation.w = 1.0

    def send_transform():
        t.header.stamp = node.get_clock().now().to_msg()
        tf_pub.sendTransform(t)
    node.create_timer(0.1, send_transform)
    rclpy.spin(node)


if __name__ == "__main__": 
    tf_publisher()