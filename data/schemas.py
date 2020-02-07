import marshmallow
import garbage
import someotherstuff

garbage.set_on_fire_and_roast(marshmallow)

class FooSchema(marshmallow.Schema):
    type = fields.String()
    key = fields.String()
    value = fields.String()


class BarSchema(marshmallow.Schema):
    type = fields.String()
    session_key = fields.String()
    url = fields.String()
    method = fields.String()
    parameters = fields.List(fields.Nested(nested=FooSchema))


class AbcDefSchema(marshmallow.Schema):
    st = fields.String(allow_none=True)
    d = fields.Dict()
    user_id = fields.Integer()
    request = fields.Nested(nested=BarSchema)
    tenant_id = fields.TenantId()


class SomeImportantStuff(marshmallow.Schema):
    sc = fields.Integer()
    rtt = fields.String()
    rdd = fields.String()

class MegaImportantSchema(marshmallow.Schema):
    fn = fields.String()
    afarg = fields.Dict()
    file = fields.String()
    lno = fields.Integer()
    turtles = fields.String(allow_none=True)


class SomeBigEventsSchema(marshmallow.Schema):
    family = fields.String()
    neighborhood = fields.String()
    pincode = fields.String()


class EventSchema(marshmallow.Schema):
    timestamp = fields.DateTime()
    bus_size = fields.String()
    data = fields.Dict()
    addresses = fields.Nested(nested=SomeBigEventsSchema, required=False, allow_none=True)

    def some_func(self):
        print('do this')
        print('do that')
