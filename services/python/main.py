import grpc
import entity_pb2
import entity_pb2_grpc

def run():
    # Assuming the server is running on localhost and port 50051
    with grpc.insecure_channel('localhost:50051') as channel:
        stub = entity_pb2_grpc.EntityProviderStub(channel)
        response = stub.GenerateEntity(entity_pb2.EntityType(type="Patata"))
        print("Entity received: \n - data: ", response.data, "\n - type: ", response.type, "\n - index: ", response.index)

if __name__ == '__main__':
    run()
