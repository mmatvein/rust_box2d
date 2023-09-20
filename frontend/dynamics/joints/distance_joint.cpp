b2Joint* World_create_distance_joint(
    b2World* self,
    b2Body* body_a,
    b2Body* body_b,
    bool collide_connected,
    b2Vec2 local_anchor_a,
    b2Vec2 local_anchor_b,
    f32 length,
    f32 stiffness,
    f32 damping
) {
    b2DistanceJointDef def;
    def.bodyA = body_a;
    def.bodyB = body_b;
    def.collideConnected = collide_connected;
    def.localAnchorA = local_anchor_a;
    def.localAnchorB = local_anchor_b;
    def.length = length;
    def.stiffness = stiffness;
    def.damping = damping;

    return self->CreateJoint(&def);
}

void DistanceJointDef_initialize(b2DistanceJointDef* self,
                                 b2Body* body_a, b2Body* body_b,
                                 const b2Vec2* anchor_a,
                                 const b2Vec2* anchor_b) {
    self->Initialize(body_a, body_b, *anchor_a, *anchor_b);
}

b2Joint* DistanceJoint_as_joint(b2DistanceJoint* self) {
    return static_cast<b2Joint*>(self);
}
b2DistanceJoint* Joint_as_distance_joint(b2Joint* self) {
    return static_cast<b2DistanceJoint*>(self);
}

const b2Vec2* DistanceJoint_get_local_anchor_a(
                                        const b2DistanceJoint* self) {
    return &self->GetLocalAnchorA();
}
const b2Vec2* DistanceJoint_get_local_anchor_b(
                                        const b2DistanceJoint* self) {
    return &self->GetLocalAnchorB();
}
void DistanceJoint_set_length(b2DistanceJoint* self, f32 length) {
    self->SetLength(length);
}
f32 DistanceJoint_get_length(const b2DistanceJoint* self) {
    return self->GetLength();
}
void DistanceJoint_set_stiffness(b2DistanceJoint* self, f32 stiffness) {
    self->SetStiffness(stiffness);
}
f32 DistanceJoint_get_stiffness(const b2DistanceJoint* self) {
    return self->GetStiffness();
}
void DistanceJoint_set_damping(b2DistanceJoint* self,
                                     f32 damping) {
    self->SetDamping(damping);
}
f32 DistanceJoint_get_damping(const b2DistanceJoint* self) {
    return self->GetDamping();
}
