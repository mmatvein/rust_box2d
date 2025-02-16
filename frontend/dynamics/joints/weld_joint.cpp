b2Joint* World_create_weld_joint(
    b2World* world,
    b2Body* body_a,
    b2Body* body_b,
    bool collide_connected,
    b2Vec2 local_anchor_a,
    b2Vec2 local_anchor_b,
    f32 reference_angle,
    f32 stiffness,
    f32 damping
) {
    b2WeldJointDef def;
    def.bodyA = body_a;
    def.bodyB = body_b;
    def.collideConnected = collide_connected;
    def.localAnchorA = local_anchor_a;
    def.localAnchorB = local_anchor_b;
    def.referenceAngle = reference_angle;
    def.stiffness = stiffness;
    def.damping = damping;

    return world->CreateJoint(&def);
}

void WeldJointDef_initialize(b2WeldJointDef* self,
                                 b2Body* body_a, b2Body* body_b,
                                 const b2Vec2* anchor) {
    self->Initialize(body_a, body_b, *anchor);
}

b2Joint* WeldJoint_as_joint(b2WeldJoint* self) {
    return static_cast<b2Joint*>(self);
}
b2WeldJoint* Joint_as_weld_joint(b2Joint* self) {
    return static_cast<b2WeldJoint*>(self);
}

const b2Vec2* WeldJoint_get_local_anchor_a(const b2WeldJoint* self) {
    return &self->GetLocalAnchorA();
}
const b2Vec2* WeldJoint_get_local_anchor_b(const b2WeldJoint* self) {
    return &self->GetLocalAnchorB();
}
f32 WeldJoint_get_reference_angle(const b2WeldJoint* self) {
    return self->GetReferenceAngle();
}
void WeldJoint_set_stiffness(b2WeldJoint* self, f32 frequency) {
    self->SetStiffness(frequency);
}
f32 WeldJoint_get_stiffness(const b2WeldJoint* self) {
    return self->GetStiffness();
}
void WeldJoint_set_damping(b2WeldJoint* self, f32 ratio) {
    self->SetDamping(ratio);
}
f32 WeldJoint_get_damping(const b2WeldJoint* self) {
    return self->GetDamping();
}
