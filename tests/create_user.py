import dataclasses
import psycopg2
import requests
import faker
from dataclasses import dataclass

faker_instance = faker.Faker()

@dataclass
class UserObject:
    name: str
    email: str
    organization: str
    phone: str
    location: str
    level: int

@dataclass
class CreateUser:
    user: UserObject
    password: str


conn = psycopg2.connect('''postgresql://postgres:cHt0UFBbszX0YK7@localhost:5432''')
cursor = conn.cursor()
cursor.execute('''DELETE FROM platform_users''')
conn.commit()

def test_create_user():
    user = UserObject(
        name=faker_instance.name(),
        email=faker_instance.email(),
        organization=faker_instance.company(),
        phone=faker_instance.phone_number()[0:5],
        location=faker_instance.country(),
        level=1
    )
    password = faker_instance.password()
    create_user = CreateUser(user=user, password=password)
    

    response = requests.post('http://localhost:5000/user/create', json=dataclasses.asdict(create_user))
    assert response.status_code == 200
    assert response.json() == {'result': 'OK'}

    cursor.execute('''SELECT password, name, email, organization, phone, location, level FROM platform_users''')
    result = cursor.fetchall()
    assert len(result) == 1
    assert result[0][0] == password
    assert result[0][1] == user.name
    assert result[0][2] == user.email
    assert result[0][3] == user.organization
    assert result[0][4] == user.phone
    assert result[0][5] == user.location
    assert result[0][6] == user.level

    response = requests.post('http://localhost:5000/user/create', json=dataclasses.asdict(create_user))
    assert response.status_code == 200
    assert response.json() == {'result': 'ALREADY_EXISTS'}

    cursor.execute('''DELETE FROM platform_users''')
    conn.commit()

    user.level = 2
    create_user = CreateUser(user=user, password=password)
    response = requests.post('http://localhost:5000/user/create', json=dataclasses.asdict(create_user))
    assert response.status_code == 200
    assert response.json() == {'result': 'ACCESS_ERROR'}    

    response = requests.post('http://localhost:5000/user/create', json=dataclasses.asdict(create_user), headers={'X-User-Level': '3'})
    assert response.status_code == 200
    assert response.json() == {'result': 'OK'}    
    

    print("test_create_user passed")

def test_get_user():
    user = UserObject(
        name=faker_instance.name(),
        email=faker_instance.email(),
        organization=faker_instance.company(),
        phone=faker_instance.phone_number()[0:5],
        location=faker_instance.country(),
        level=1
    )
    password = faker_instance.password()
    create_user = CreateUser(user=user, password=password)
    response = requests.post('http://localhost:5000/user/create', json=dataclasses.asdict(create_user))
    assert response.status_code == 200
    assert response.json() == {'result': 'OK'}
    cursor.execute('''SELECT password, name, email, organization, phone, location, level FROM platform_users''')

    response = requests.get('http://localhost:5000/user/get', json={"email": faker_instance.email()}, headers={'X-User-Level': '1'})
    assert response.status_code == 200
    assert response.json() == {'result': 'DOESNT_EXISTS'}

    response = requests.get('http://localhost:5000/user/get', json={"email": faker_instance.email()}, headers={})
    assert response.status_code == 200
    assert response.json() == {'result': 'ACCESS_ERROR'}

    response = requests.get('http://localhost:5000/user/get', json={"email": user.email}, headers={'X-User-Level': '1'})
    assert response.status_code == 200
    
    result = cursor.execute('''SELECT * FROM platform_users''')

    assert response.json() == {
        'user': {
            'name': user.name,
            'email': user.email,
            'organization': user.organization,
            'phone': user.phone,
            'location': user.location,
            'level': user.level
        }
    }


    print("test_get_user passed")


def test_user_auth():
    user = UserObject(
        name=faker_instance.name(),
        email=faker_instance.email(),
        organization=faker_instance.company(),
        phone=faker_instance.phone_number()[0:5],
        location=faker_instance.country(),
        level=1
    )
    password = faker_instance.password()
    create_user = CreateUser(user=user, password=password)
    response = requests.post('http://localhost:5000/user/create', json=dataclasses.asdict(create_user))
    assert response.status_code == 200
    assert response.json() == {'result': 'OK'}

    response = requests.post('http://localhost:5000/user/auth', json={"email": user.email, "password": password})
    assert response.status_code == 200
    
    response = response.json()
    assert len(response.keys()) == 3
    assert response['level'] == user.level
    assert len(response['token']) == 32
    assert response['email'] == user.email

    response = requests.post('http://localhost:5000/user/auth', json={"email": user.email, "password": faker_instance.password()})
    assert response.status_code == 200
    assert response.json() == {'result': 'DOESNT_EXISTS'}

    response = requests.post('http://localhost:5000/user/auth', json={"email": faker_instance.email(), "password": faker_instance.password()})
    assert response.status_code == 200
    assert response.json() == {'result': 'DOESNT_EXISTS'}

    print("test_user_auth passed")
    
test_create_user()
test_get_user()
test_user_auth()
