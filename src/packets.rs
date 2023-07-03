use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;

pub trait GettableEndPoints {
    fn get_source(&self) -> String;
    fn get_destination(&self) -> String;
    fn get_payload(&self) -> &[u8];
}

impl GettableEndPoints for Ipv4Packet<'_> {
    fn get_source(&self) -> String {
        self.get_source().to_string()
    }
    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }
    fn get_payload(&self) -> &[u8] {
        self.payload()
    }
}

impl GettableEndPoints for Ipv6Packet<'_> {
    fn get_source(&self) -> String {
        self.get_source().to_string()
    }
    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }
    fn get_payload(&self) -> &[u8] {
        self.payload()
    }
}

impl GettableEndPoints for TcpPacket<'_> {
    fn get_source(&self) -> String {
        self.get_source().to_string()
    }
    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }
    fn get_payload(&self) -> &[u8] {
        self.payload()
    }
}

impl GettableEndPoints for UdpPacket<'_> {
    fn get_source(&self) -> String {
        self.get_source().to_string()
    }
    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }
    fn get_payload(&self) -> &[u8] {
        self.payload()
    }
}
