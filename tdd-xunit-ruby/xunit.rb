class TestCase
  def initialize(name)
    @name = name
  end

  def run
    send(@name)
  end
end

class WasRun < TestCase
  def initialize(name)
    super
    @was_run = false
  end

  def was_run?
    @was_run
  end

  def test_method
    @was_run = true
  end
end
