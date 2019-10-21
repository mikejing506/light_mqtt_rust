// MQTT Control Packet Type Byte0[7..4]
const MQTT_Control_Packet_type_Reserved = 0;
const MQTT_Control_Packet_type_CONNECT = 1;
const MQTT_Control_Packet_type_CONNACK = 2;
const MQTT_Control_Packet_type_PUBLISH = 3;
const MQTT_Control_Packet_type_PUBACK = 4;
const MQTT_Control_Packet_type_PUBREC = 5;
const MQTT_Control_Packet_type_PUBREL = 6;
const MQTT_Control_Packet_type_PUBCOMP = 7;
const MQTT_Control_Packet_type_SUBSCRIBE = 8;
const MQTT_Control_Packet_type_SUBACK = 9;
const MQTT_Control_Packet_type_UNSUBSCRIBE = 10;
const MQTT_Control_Packet_type_UNSUBACK = 11;
const MQTT_Control_Packet_type_PINGREQ = 12;
const MQTT_Control_Packet_type_PINGRESP = 13;
const MQTT_Control_Packet_type_DISCONNECT = 14;
const MQTT_Control_Packet_type_Reserved_1 = 15;

//MQTT Control Packet Flag Byte0[3..0]
const MQTT_Control_Packet_flags_CONNECT = 0b0000;
const MQTT_Control_Packet_flags_CONNACK = 0b0000;
// const MQTT_Control_Packet_flags_PUBLISH = 0b0000;//rewrite in QOS generate
const MQTT_Control_Packet_flags_PUBACK = 0b0000;
const MQTT_Control_Packet_flags_PUBREC = 0b0000;
const MQTT_Control_Packet_flags_PUBREL = 0b0010;
const MQTT_Control_Packet_flags_PUBCOMP = 0b0000;
const MQTT_Control_Packet_flags_SUBSCRIBE = 0b0010;
const MQTT_Control_Packet_flags_SUBACK = 0b0000;
const MQTT_Control_Packet_flags_UNSUBSCRIBE = 0b0010;
const MQTT_Control_Packet_flags_UNSUBACK = 0b0000;
const MQTT_Control_Packet_flags_PINGREQ = 0b0000;
const MQTT_Control_Packet_flags_PINGRESP = 0b0000;
const MQTT_Control_Packet_flags_DISCONNECT = 0b0000;

